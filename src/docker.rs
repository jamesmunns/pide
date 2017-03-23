use std::env::current_dir;
use std::process::Command;
use std::path::Path;

use types::{PideName, PideError};

pub fn get_image_name(pide: &PideName) -> Result<String, PideError> {
    let pide_str = pide.to_string();

    let cmd_out = Command::new("docker")
        .arg("ps")
        .output()?
        .stdout;

    let cmd_out_str = String::from_utf8_lossy(&cmd_out);

    cmd_out_str.lines()
        .filter(|x| x.contains(&pide_str))
        .map(|y| y.split(' ').last().expect("docker parse error").to_string()) // TODO: ? in a map?
        .nth(0).ok_or(PideError::Unknown)
}

pub fn ssh_attach(id: &str) -> Result<(), PideError> {
    let _ = Command::new("docker").arg("exec")
        .arg("-it")
        .arg(id)
        .arg("/bin/bash")
        .spawn()?
        .wait()?;

    Ok(())
}

pub fn run(name: &str, label: &str) -> Result<(), PideError> {
    let _ = Command::new("docker")
        .arg("run")
        .arg("-v")
        .arg(format!("{}:/host",
                     current_dir()?
                         .to_str()
                         .ok_or(PideError::Unknown)?))
        .arg("--name")
        .arg(name)
        .arg("-it")
        .arg(label)
        .arg("/bin/bash")
        .spawn()?
        .wait();

    Ok(())
}

pub fn commit(name: &str, label: &str) -> Result<(), PideError> {
    let _ = Command::new("docker").arg("commit")
        .arg(name)
        .arg(label)
        .output()?;

    Ok(())
}

pub fn build(label: &str, dockerfile: &str, working_path: &Path) -> Result<String, PideError> {
    let build_cmd = Command::new("docker").arg("build")
        .arg("-t")
        .arg(label)
        .arg("-f")
        .arg(dockerfile)
        .arg(working_path)
        .output()?;

    let image_id = String::from_utf8_lossy(&build_cmd.stdout)
        .lines()
        .last()
        .ok_or(PideError::ParseError)?
        .split(' ')
        .last()
        .ok_or(PideError::ParseError)?
        .to_string();

    Ok(image_id)
}
