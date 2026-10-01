use std::{
    fs::{File, Metadata},
    io::{self, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StartPosition {
    Beginning,
    End,
}

/// Incrementally reads complete lines from a log file.
///
/// The tailer stores a byte offset and only reads bytes appended since the
/// previous poll. Bytes after the last newline remain buffered until a later
/// poll completes that line.
pub struct LogTailer {
    path: PathBuf,
    file: File,
    identity: FileIdentity,
    offset: u64,
    generation: u64,
    partial_line: Vec<u8>,
    bytes_read_last_poll: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FileIdentity {
    #[cfg(unix)]
    device: u64,
    #[cfg(unix)]
    inode: u64,
    #[cfg(windows)]
    created: u64,
    #[cfg(not(any(unix, windows)))]
    created: Option<std::time::SystemTime>,
}

impl FileIdentity {
    fn from_metadata(metadata: &Metadata) -> Self {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            Self {
                device: metadata.dev(),
                inode: metadata.ino(),
            }
        }
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt;
            Self {
                created: metadata.creation_time(),
            }
        }
        #[cfg(not(any(unix, windows)))]
        {
            Self {
                created: metadata.created().ok(),
            }
        }
    }
}

impl LogTailer {
    pub fn open(path: impl AsRef<Path>, start: StartPosition) -> io::Result<Self> {
        let path = path.as_ref().to_owned();
        let mut file = File::open(&path)?;
        let metadata = file.metadata()?;
        let identity = FileIdentity::from_metadata(&metadata);
        let offset = match start {
            StartPosition::Beginning => 0,
            StartPosition::End => metadata.len(),
        };
        file.seek(SeekFrom::Start(offset))?;
        Ok(Self {
            path,
            file,
            identity,
            offset,
            generation: 0,
            partial_line: Vec::new(),
            bytes_read_last_poll: 0,
        })
    }

    /// Reads newly appended bytes and returns complete decoded lines.
    pub fn poll(&mut self) -> io::Result<Vec<String>> {
        self.bytes_read_last_poll = 0;
        let path_metadata = std::fs::metadata(&self.path)?;
        let path_identity = FileIdentity::from_metadata(&path_metadata);

        if path_identity != self.identity || path_metadata.len() < self.offset {
            self.reopen(StartPosition::Beginning)?;
        }

        self.file.seek(SeekFrom::Start(self.offset))?;
        let mut appended = Vec::new();
        self.bytes_read_last_poll = self.file.read_to_end(&mut appended)?;
        self.offset += self.bytes_read_last_poll as u64;
        self.partial_line.extend(appended);
        Ok(self.take_complete_lines())
    }

    pub fn bytes_read_last_poll(&self) -> usize {
        self.bytes_read_last_poll
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    /// Increments whenever the path is reopened after replacement or truncation.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    fn reopen(&mut self, start: StartPosition) -> io::Result<()> {
        self.file = File::open(&self.path)?;
        let metadata = self.file.metadata()?;
        self.identity = FileIdentity::from_metadata(&metadata);
        self.offset = match start {
            StartPosition::Beginning => 0,
            StartPosition::End => metadata.len(),
        };
        self.file.seek(SeekFrom::Start(self.offset))?;
        self.partial_line.clear();
        self.generation += 1;
        Ok(())
    }

    fn take_complete_lines(&mut self) -> Vec<String> {
        let mut lines = Vec::new();
        let mut start = 0;
        for end in 0..self.partial_line.len() {
            if self.partial_line[end] == b'\n' {
                let mut line = self.partial_line[start..end].to_vec();
                if line.last() == Some(&b'\r') {
                    line.pop();
                }
                lines.push(String::from_utf8_lossy(&line).into_owned());
                start = end + 1;
            }
        }
        if start > 0 {
            self.partial_line.drain(..start);
        }
        lines
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Write};

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn starts_at_end_and_reads_only_new_bytes() {
        let root = tempdir().expect("temp directory");
        let path = root.path().join("WizardClient.log");
        fs::write(&path, "old line\n").expect("write initial content");
        let mut tailer = LogTailer::open(&path, StartPosition::End).expect("open tailer");

        assert!(tailer.poll().expect("initial poll").is_empty());
        assert_eq!(tailer.bytes_read_last_poll(), 0);

        fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .expect("open append")
            .write_all(b"new line\n")
            .expect("append data");
        assert_eq!(tailer.poll().expect("poll").as_slice(), ["new line"]);
        assert_eq!(tailer.bytes_read_last_poll(), b"new line\n".len());
    }

    #[test]
    fn buffers_partial_line_and_handles_crlf() {
        let root = tempdir().expect("temp directory");
        let path = root.path().join("WizardClient.log");
        fs::write(&path, "").expect("create log");
        let mut tailer = LogTailer::open(&path, StartPosition::Beginning).expect("open tailer");

        fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .expect("open append")
            .write_all(b"half")
            .expect("append partial");
        assert!(tailer.poll().expect("poll partial").is_empty());
        assert_eq!(tailer.offset(), 4);

        fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .expect("open append")
            .write_all(b" line\r\n")
            .expect("complete line");
        assert_eq!(
            tailer.poll().expect("poll complete").as_slice(),
            ["half line"]
        );
    }

    #[test]
    fn reopens_after_truncation_and_replacement() {
        let root = tempdir().expect("temp directory");
        let path = root.path().join("WizardClient.log");
        fs::write(&path, "initial-complete-line\n").expect("write initial");
        let mut tailer = LogTailer::open(&path, StartPosition::Beginning).expect("open tailer");
        assert_eq!(
            tailer.poll().expect("read initial").as_slice(),
            ["initial-complete-line"]
        );

        fs::write(&path, "short\n").expect("truncate");
        assert_eq!(
            tailer.poll().expect("read truncation").as_slice(),
            ["short"]
        );
        assert_eq!(tailer.generation(), 1);

        let rotated = root.path().join("WizardClient.old");
        fs::rename(&path, &rotated).expect("rename old log");
        fs::write(&path, "replacement\n").expect("new log");
        assert_eq!(
            tailer.poll().expect("read replacement").as_slice(),
            ["replacement"]
        );
        assert_eq!(tailer.generation(), 2);
    }
}
