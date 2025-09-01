//use std::process::Command;

const UNIVERSAL_IMAGE: &str = "s4s_runner";

pub const JAVA_IMAGE: &str = UNIVERSAL_IMAGE;
pub const PYTHON_IMAGE: &str = UNIVERSAL_IMAGE;
pub const CPP_IMAGE: &str = UNIVERSAL_IMAGE;

pub const IMAGES: [&str; 3] = [JAVA_IMAGE, PYTHON_IMAGE, CPP_IMAGE];

pub fn pull_required_images() {
    // OBSOLETE
    /*if let Err(err) = Command::new("docker").args(["pull", UNIVERSAL_IMAGE]).status() {
        println!("Error while pulling {UNIVERSAL_IMAGE}: {err:#?}");
    }*/
}
