//! 12 (3x) - Move-only тип. Эталонное решение.

pub struct FileHandle {
    fd: u32,
    path: String,
}

impl FileHandle {
    pub fn open(fd: u32, path: &str) -> FileHandle {
        FileHandle {
            fd,
            path: path.to_string(),
        }
    }

    pub fn descriptor(&self) -> u32 {
        self.fd
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn close(self) -> String {
        self.path
    }
}
