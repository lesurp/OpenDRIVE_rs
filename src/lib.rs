mod automagic;
mod open_drive;

pub use automagic::open_drive::*;
pub use automagic::*;
pub use open_drive::OpenDriveError;

#[cfg(test)]
mod tests {
    use crate::OpenDrive;

    #[test]
    fn test_loading_town01() {
        let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("test/Town01.xodr");
        let _ = OpenDrive::from_path(dir).unwrap();
    }
}
