use crate::config::config;
use std::process::Command;

pub fn pull_runner_image() {
    if config().runner_image.ends_with(":dev") {
        println!("Runner image has tag ':dev', skipping pull.");
        return;
    }
    let image = config().runner_image.clone();
    if let Err(err) = Command::new("docker").args(["pull", &image]).status() {
        println!("Error while pulling {image}: {err:#?}");
    }
}
