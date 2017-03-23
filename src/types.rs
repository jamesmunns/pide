use std::io::prelude::*;
use std::fs::File;
use uuid::Uuid;
use toml;
use std::fmt;
use std::io;

// use std::convert::From;
use std::error::Error;

#[derive(Debug)]
pub enum PideError {
    IoError(String),
    ParseError,
    TomlSerError(String),
    TomlDesError(String),
    Unknown,
}

impl fmt::Display for PideError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pide Error: {}", self.description())
    }
}

impl Error for PideError {
    fn description(&self) -> &str {
        match *self {
            PideError::TomlSerError(ref msg) => msg,
            PideError::TomlDesError(ref msg) => msg,
            PideError::IoError(ref msg) => msg,
            PideError::ParseError => "an error occurred while parsing",
            PideError::Unknown => "an unknown error occurred",
        }
    }
}

impl From<toml::ser::Error> for PideError {
    fn from(err: toml::ser::Error) -> PideError {
        PideError::TomlSerError(format!("{}", err))
    }
}

impl From<toml::de::Error> for PideError {
    fn from(err: toml::de::Error) -> PideError {
        PideError::TomlDesError(format!("{}", err))
    }
}

impl From<io::Error> for PideError {
    fn from(err: io::Error) -> PideError {
        PideError::IoError(format!("{}", err))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PideName {
    repo: String,
    id: Uuid,
}

impl PideName {
    pub fn new() -> Self {
        Self {
            repo: "pide".to_string(),
            id: Uuid::new_v4(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", &self.repo, self.id.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PideFile {
    pub image_id: String,
    pub original_dockerfile: String, // TODO: not a string?
    pub original_workingpath: String, // TODO: not a string?
    pub pide_version: String, // TODO: not a string?

    // Tables go last
    pub name: PideName,
}

impl PideFile {
    pub fn to_string(&self) -> Result<String, PideError> {
        Ok(toml::to_string(self)?)
    }

    pub fn to_file(&self, path: &str) -> Result<(), PideError> {
        let mut file = File::create(path)?;
        let _ = file.write_all(&self.to_string()?.into_bytes())?;
        Ok(())
    }

    pub fn from_file(open_path: &str) -> Result<Self, PideError> {

        let mut file = File::open(open_path)?;
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        Ok(toml::from_str(&contents)?)
    }
}
