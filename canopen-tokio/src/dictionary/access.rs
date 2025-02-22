use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct AccessType {
    pub read_access: bool,
    pub write_access: bool,
}

impl AccessType {
    pub const NONE: AccessType = AccessType {
        read_access: false,
        write_access: false,
    };

    pub const READ_WRITE: AccessType = AccessType {
        read_access: true,
        write_access: true,
    };

    pub const READ_ONLY: AccessType = AccessType {
        read_access: true,
        write_access: false,
    };

    pub const WRITE_ONLY: AccessType = AccessType {
        read_access: false,
        write_access: true,
    };

    pub fn is_readable(&self) -> bool {
        self.read_access
    }

    pub fn is_writable(&self) -> bool {
        self.write_access
    }
}

impl FromStr for AccessType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let keyword = s.to_lowercase();

        Ok(match keyword.as_str() {
            "rw" => AccessType::READ_WRITE,
            "ro" => AccessType::READ_ONLY,
            "wo" => AccessType::WRITE_ONLY,
            _ => AccessType::NONE,
        })
    }
}
