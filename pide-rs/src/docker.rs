use std::env::current_dir;
use std::process::Command;
use std::path::Path;

use types::PideName;

pub fn get_image_name(pide: &PideName) -> Option<String> {
    let pide_str = pide.to_string();

    let cmd_out = Command::new("docker")
        .arg("ps")
        .output()
        .expect("failed to run docker")
        .stdout;

    let cmd_out_str = String::from_utf8_lossy(&cmd_out);

    cmd_out_str.lines()
        .filter(|x| x.contains(&pide_str))
        .map(|y| y.split(' ').last().expect("docker parse error").to_string())
        .nth(0)
}

pub fn ssh_attach(id: &str) {
    let _ = Command::new("docker")
        .arg("exec")
        .arg("-it")
        .arg(id)
        .arg("/bin/bash")
        .spawn()
        .expect("Docker failed to launch")
        .wait()
        .expect("Docker failed at runtime");
}

pub fn run(name: &str, label: &str) {
    let _ = Command::new("docker")
        .arg("run")
        .arg("-v")
        .arg(format!("{}:/host",
                     current_dir()
                         .expect("failed to get cwd")
                         .to_str()
                         .expect("bad utf-8 in cwd")))
        .arg("--name")
        .arg(name)
        .arg("-it")
        .arg(label)
        .arg("/bin/bash")
        .spawn()
        .expect("Docker failed to launch")
        .wait()
        .expect("Docker failed at runtime");
}

pub fn commit(name: &str, label: &str) {
    let _ = Command::new("docker")
        .arg("commit")
        .arg(name)
        .arg(label)
        .output()
        .expect("Failed to commit docker changes");
}

pub fn build(label: &str, dockerfile: &str, working_path: &Path) -> String {
    let build_cmd = Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg(label)
        .arg("-f")
        .arg(dockerfile)
        .arg(working_path)
        .output()
        .expect("Failed to build base docker image!");

    let image_id = String::from_utf8_lossy(&build_cmd.stdout)
        .lines()
        .last()
        .expect("Unexpected Docker Output!")
        .split(' ')
        .last()
        .expect("Failed to decode image id")
        .to_string();

    image_id
}
