use std::time::{Duration, Instant};

use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity};

use crate::presence::Presence;

pub trait DiscordTransport {
    fn connect(&mut self) -> Result<(), String>;
    fn publish(&mut self, presence: &Presence) -> Result<(), String>;
    fn clear(&mut self) -> Result<(), String>;
    fn disconnect(&mut self);
}

pub struct IpcTransport {
    application_id: String,
    client: Option<DiscordIpcClient>,
}

impl IpcTransport {
    pub fn new(application_id: String) -> Self {
        Self {
            application_id,
            client: None,
        }
    }
}

impl DiscordTransport for IpcTransport {
    fn connect(&mut self) -> Result<(), String> {
        self.disconnect();
        let mut client = DiscordIpcClient::new(&self.application_id);
        client.connect().map_err(|error| error.to_string())?;
        self.client = Some(client);
        Ok(())
    }

    fn publish(&mut self, presence: &Presence) -> Result<(), String> {
        let Some(client) = self.client.as_mut() else {
            return Err("Discord IPC is not connected".into());
        };
        client
            .set_activity(activity_payload(presence))
            .map_err(|error| error.to_string())?;
        // SET_ACTIVITY is acknowledged with the command nonce. Reading the
        // response detects a Discord process restart even when a stale named
        // pipe still accepts writes.
        client.recv().map(|_| ()).map_err(|error| error.to_string())
    }

    fn clear(&mut self) -> Result<(), String> {
        let Some(client) = self.client.as_mut() else {
            return Ok(());
        };
        client.clear_activity().map_err(|error| error.to_string())?;
        client.recv().map(|_| ()).map_err(|error| error.to_string())
    }

    fn disconnect(&mut self) {
        if let Some(mut client) = self.client.take() {
            let _ = client.close();
        }
    }
}

fn activity_payload(presence: &Presence) -> activity::Activity<'_> {
    let mut activity = activity::Activity::new().name("Wizard101");
    if let Some(details) = &presence.details {
        activity = activity.details(details.as_str());
    }
    if let Some(state) = &presence.state {
        activity = activity.state(state.as_str());
    }
    if let Some(start) = presence.start_unix_seconds {
        activity = activity.timestamps(activity::Timestamps::new().start(start));
    }
    let mut assets = activity::Assets::new();
    let mut has_assets = false;
    if let Some(image) = &presence.large_image {
        assets = assets.large_image(image.as_str());
        has_assets = true;
    }
    if let Some(text) = &presence.large_text {
        assets = assets.large_text(text.as_str());
        has_assets = true;
    }
    if let Some(image) = &presence.small_image {
        assets = assets.small_image(image.as_str());
        has_assets = true;
    }
    if let Some(text) = &presence.small_text {
        assets = assets.small_text(text.as_str());
        has_assets = true;
    }
    if has_assets {
        activity = activity.assets(assets);
    }
    activity
}

const RETRY_INITIAL: Duration = Duration::from_secs(1);
const RETRY_MAX: Duration = Duration::from_secs(60);
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// Owns delivery state; each tick replaces the desired payload with the latest model.
pub struct PresencePublisher<T: DiscordTransport> {
    transport: T,
    connected: bool,
    sent: Option<Presence>,
    last_sent_at: Option<Instant>,
    retry_at: Option<Instant>,
    failures: u32,
}

impl<T: DiscordTransport> PresencePublisher<T> {
    pub fn new(transport: T) -> Self {
        Self {
            transport,
            connected: false,
            sent: None,
            last_sent_at: None,
            retry_at: None,
            failures: 0,
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Returns a new transport error for diagnostics; never stops the watcher.
    pub fn tick(&mut self, desired: Option<&Presence>, now: Instant) -> Option<String> {
        if desired.is_none() {
            self.retry_at = None;
            self.failures = 0;
            if self.connected && self.sent.is_some() {
                if let Err(error) = self.transport.clear() {
                    return Some(self.failed(error, now));
                }
                self.sent = None;
                self.last_sent_at = None;
            }
            return None;
        }

        if !self.connected {
            if self.retry_at.is_some_and(|retry_at| now < retry_at) {
                return None;
            }
            if let Err(error) = self.transport.connect() {
                return Some(self.failed(error, now));
            }
            self.connected = true;
            self.retry_at = None;
            self.failures = 0;
            self.sent = None;
            self.last_sent_at = None;
        }

        let desired = desired.expect("checked above");
        if self.sent.as_ref() == Some(desired)
            && self.last_sent_at.is_some_and(|last_sent_at| {
                now.checked_duration_since(last_sent_at)
                    .is_some_and(|age| age < HEARTBEAT_INTERVAL)
            })
        {
            return None;
        }
        if let Err(error) = self.transport.publish(desired) {
            return Some(self.failed(error, now));
        }
        self.sent = Some(desired.clone());
        self.last_sent_at = Some(now);
        None
    }

    fn failed(&mut self, error: String, now: Instant) -> String {
        self.transport.disconnect();
        self.connected = false;
        self.sent = None;
        self.last_sent_at = None;
        self.failures = self.failures.saturating_add(1);
        let exponent = self.failures.saturating_sub(1).min(6);
        let delay = RETRY_INITIAL.saturating_mul(1 << exponent).min(RETRY_MAX);
        self.retry_at = now.checked_add(delay);
        error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Fake {
        calls: Vec<&'static str>,
        fail_connect: bool,
        fail_publish: bool,
        fail_clear: bool,
    }

    impl DiscordTransport for Fake {
        fn connect(&mut self) -> Result<(), String> {
            self.calls.push("connect");
            if self.fail_connect {
                Err("connect failed".into())
            } else {
                Ok(())
            }
        }
        fn publish(&mut self, _: &Presence) -> Result<(), String> {
            self.calls.push("publish");
            if self.fail_publish {
                Err("publish failed".into())
            } else {
                Ok(())
            }
        }
        fn clear(&mut self) -> Result<(), String> {
            self.calls.push("clear");
            if self.fail_clear {
                Err("clear failed".into())
            } else {
                Ok(())
            }
        }
        fn disconnect(&mut self) {
            self.calls.push("disconnect");
        }
    }

    fn presence(state: &str) -> Presence {
        Presence {
            details: None,
            state: Some(state.into()),
            start_unix_seconds: None,
            large_image: None,
            large_text: None,
            small_image: None,
            small_text: None,
        }
    }

    #[test]
    fn adapter_serializes_only_present_discord_fields_in_unix_seconds() {
        let mut model = presence("Zafaria");
        model.details = Some("Ravenwood".into());
        model.start_unix_seconds = Some(1_800_000_000);
        model.large_image = Some("wizard_city_png".into());
        model.large_text = Some("Wizard City".into());
        model.small_image = Some("wizrust101_rpc".into());
        model.small_text = Some("WizRust101-RPC".into());
        let json = serde_json::to_value(activity_payload(&model)).expect("serialize activity");
        assert_eq!(json["name"], "Wizard101");
        assert_eq!(json["details"], "Ravenwood");
        assert_eq!(json["state"], "Zafaria");
        assert_eq!(json["timestamps"]["start"], 1_800_000_000_i64);
        assert_eq!(json["assets"]["large_image"], "wizard_city_png");
        assert_eq!(json["assets"]["large_text"], "Wizard City");
        assert_eq!(json["assets"]["small_image"], "wizrust101_rpc");
        assert_eq!(json["assets"]["small_text"], "WizRust101-RPC");
        let bare = serde_json::to_value(activity_payload(&presence("Zafaria"))).unwrap();
        assert!(bare.get("details").is_none());
        assert!(bare.get("timestamps").is_none());
        assert!(bare.get("assets").is_none());
    }

    #[test]
    fn final_discord_payload_keeps_large_and_small_assets_in_their_roles() {
        let cases = [
            ("House", Some("Botanical Gardens"), None, "house"),
            (
                "normal",
                Some("The Commons"),
                Some("Wizard City"),
                "wizardcity",
            ),
            ("unknown-world", Some("Court_Test"), None, "wizard101"),
            ("both-unknown", None, None, "wizard101"),
        ];

        for (case, details, state, large_image) in cases {
            let mut model = presence("ignored");
            model.details = details.map(str::to_owned);
            model.state = state.map(str::to_owned);
            model.start_unix_seconds = Some(1_800_000_000);
            model.large_image = Some(large_image.into());
            model.large_text = (case == "normal").then(|| "Wizard City".into());
            model.small_image = Some("wizrust101_rpc".into());
            model.small_text = Some("WizRust101-RPC".into());

            let json = serde_json::to_value(activity_payload(&model))
                .expect("serialize final Discord activity");
            assert_eq!(json["assets"]["large_image"], large_image, "{case}");
            assert_eq!(json["assets"]["small_image"], "wizrust101_rpc", "{case}");
            assert_eq!(
                json.get("details").and_then(|value| value.as_str()),
                details
            );
            assert_eq!(json.get("state").and_then(|value| value.as_str()), state);
            assert_eq!(json["timestamps"]["start"], 1_800_000_000_i64);
            if case == "House" {
                assert!(json.to_string().find("House").is_none());
                assert!(
                    json.get("assets")
                        .and_then(|a| a.get("large_text"))
                        .is_none()
                );
            }
            if case == "both-unknown" {
                assert!(json.get("details").is_none());
                assert!(json.get("state").is_none());
            }
        }
    }

    #[test]
    fn deduplicates_updates_and_clears_only_when_needed() {
        let now = Instant::now();
        let mut publisher = PresencePublisher::new(Fake::default());
        assert_eq!(publisher.tick(Some(&presence("one")), now), None);
        assert_eq!(publisher.tick(Some(&presence("one")), now), None);
        assert_eq!(publisher.tick(Some(&presence("two")), now), None);
        assert_eq!(publisher.tick(None, now), None);
        assert_eq!(publisher.tick(None, now), None);
        assert_eq!(
            publisher.transport.calls,
            ["connect", "publish", "publish", "clear"]
        );
    }

    #[test]
    fn unchanged_activity_gets_a_periodic_heartbeat() {
        let now = Instant::now();
        let mut publisher = PresencePublisher::new(Fake::default());
        publisher.tick(Some(&presence("one")), now);
        publisher.tick(Some(&presence("one")), now + Duration::from_secs(4));
        publisher.tick(Some(&presence("one")), now + Duration::from_secs(5));
        assert_eq!(publisher.transport.calls, ["connect", "publish", "publish"]);
    }

    #[test]
    fn heartbeat_detects_discord_restart_and_republishes_retained_presence() {
        let now = Instant::now();
        let current = presence("Zafaria");
        let mut publisher = PresencePublisher::new(Fake::default());
        assert_eq!(publisher.tick(Some(&current), now), None);

        // A restarted Discord instance closes the old pipe. The command
        // acknowledgement read fails on the next heartbeat.
        publisher.transport.fail_publish = true;
        assert_eq!(
            publisher.tick(Some(&current), now + Duration::from_secs(6)),
            Some("publish failed".into())
        );
        publisher.transport.fail_publish = false;
        assert_eq!(
            publisher.tick(Some(&current), now + Duration::from_secs(7)),
            None
        );
        assert_eq!(
            publisher.transport.calls,
            [
                "connect",
                "publish",
                "publish",
                "disconnect",
                "connect",
                "publish"
            ]
        );
    }

    #[test]
    fn connect_error_backs_off_then_republishes_latest_value() {
        let now = Instant::now();
        let mut publisher = PresencePublisher::new(Fake {
            fail_connect: true,
            ..Default::default()
        });
        assert_eq!(
            publisher.tick(Some(&presence("one")), now),
            Some("connect failed".into())
        );
        publisher.transport.fail_connect = false;
        assert_eq!(publisher.tick(Some(&presence("two")), now), None);
        assert_eq!(
            publisher.tick(Some(&presence("two")), now + Duration::from_secs(1)),
            None
        );
        assert_eq!(
            publisher.transport.calls,
            ["connect", "disconnect", "connect", "publish"]
        );
    }

    #[test]
    fn publish_error_disconnects_and_retry_restores_activity() {
        let now = Instant::now();
        let mut publisher = PresencePublisher::new(Fake {
            fail_publish: true,
            ..Default::default()
        });
        assert_eq!(
            publisher.tick(Some(&presence("one")), now),
            Some("publish failed".into())
        );
        publisher.transport.fail_publish = false;
        assert_eq!(
            publisher.tick(Some(&presence("two")), now + Duration::from_secs(1)),
            None
        );
        assert_eq!(
            publisher.transport.calls,
            ["connect", "publish", "disconnect", "connect", "publish"]
        );
    }

    #[test]
    fn clear_failure_disconnects_and_no_connection_is_needed_for_none() {
        let now = Instant::now();
        let mut publisher = PresencePublisher::new(Fake {
            fail_clear: true,
            ..Default::default()
        });
        publisher.tick(Some(&presence("one")), now);
        assert_eq!(publisher.tick(None, now), Some("clear failed".into()));
        assert_eq!(publisher.tick(None, now + Duration::from_secs(1)), None);
        assert_eq!(
            publisher.transport.calls,
            ["connect", "publish", "clear", "disconnect"]
        );
    }
}
