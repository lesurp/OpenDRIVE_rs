use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use super::OpenDrive;

#[derive(Debug)]
pub enum OpenDriveError {
    OpeningFile(std::io::Error),
    ParsingXml(String),
}

impl From<std::io::Error> for OpenDriveError {
    fn from(e: std::io::Error) -> Self {
        OpenDriveError::OpeningFile(e)
    }
}

impl From<String> for OpenDriveError {
    fn from(e: String) -> Self {
        OpenDriveError::ParsingXml(e)
    }
}

impl Error for OpenDriveError {}

impl Display for OpenDriveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenDriveError::ParsingXml(s) => write!(
                f,
                "Error parsing the XODR data - might be due to faulty datastructures... Error: {}",
                s
            ),
            OpenDriveError::OpeningFile(e) => write!(f, "Error opening XODR file: {}", e),
        }
    }
}

impl OpenDrive {
    pub fn from_reader<R: Read>(reader: R) -> Result<OpenDrive, OpenDriveError> {
        Ok(yaserde::de::from_reader(reader)?)
    }

    pub fn from_path<P: AsRef<Path>>(file_path: P) -> Result<OpenDrive, OpenDriveError> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        OpenDrive::from_reader(reader)
    }

    pub fn from_string<S: AsRef<str>>(s: S) -> Result<OpenDrive, OpenDriveError> {
        Ok(yaserde::de::from_str(s.as_ref())?)
    }
}
