use std::{ process::Command, str };

pub fn path_traversal() -> Vec<String> {
    let output = Command::new("lsblk")
            .arg("--output")
            .arg("MOUNTPOINT")
            .output()
            .expect("Something went wrong when using command (maybe lsblk not supported)");
    str::from_utf8(&output.stdout).unwrap().split('\n').filter(|s| s.contains("/media")).map(|s| s.to_owned()).collect()
}