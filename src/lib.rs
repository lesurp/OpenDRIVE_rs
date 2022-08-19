mod automagic;
mod open_drive;

// TODO: Why do I need to export this manually?
// The wildcard appears to only export child modules...
pub use automagic::{open_drive::RoadType, open_drive::*, OpenDrive};
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

    #[test]
    fn test_imports() {
        use crate::RoadType;
        let r = RoadType::default();
    }
}
