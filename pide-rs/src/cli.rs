use clap::{App, SubCommand, ArgMatches, Arg};
use uuid::Uuid;
use std::path::Path;
use std::process::Command;

use toml;
use std::io::prelude::*;
use std::fs::File;
use std::env::current_dir;



pub fn parse_cli() {
    let matches = App::new("pide: Persistent Isolated Development Environments (in Docker)")
        .author(crate_authors!())
        .version(crate_version!())

        // "pide init <dockerfile>"
        .subcommand(SubCommand::with_name("init")
            .about("Initialize pide here")
            .arg(Arg::with_name("Dockerfile")
                .help("Dockerfile to initialize this environment with")
                .required(true)))

        // "pide resume"
        .subcommand(SubCommand::with_name("resume")
            .about("Resume an existing pide"))

        .get_matches();

    match matches.subcommand() {
        ("init", Some(args)) => {
            let dockerfile = args.value_of("Dockerfile").expect("Missing required argument?");
            init(dockerfile);
        }
        ("resume", _) => {
            resume();
        }
        _ => println!("Hallo, welt!"),
    }
}

fn resume() {
    // Load pide file
    let pide_data = PideFile::from_file(".pide");
    assert_eq!(pide_data.pide_version, "0.2.0", "Version Mismatch!");

    let output = Command::new("docker")
        .arg("images")
        .output()
        .expect("Failed to get existing images")
        .stdout;

    let existing_images = String::from_utf8_lossy(&output);

    let dockerfile = pide_data.original_dockerfile;
    let id: String = pide_data.name.id.to_string();
    let name_str = pide_data.name.to_string();

    let name = if !existing_images.contains(&id) {
        println!("Running for the first time...");
        &pide_data.image_id
    } else {
        println!("Resuming...");
        &name_str
    };

    let temp_name = Uuid::new_v4().to_string();

    let process = Command::new("docker")
        .arg("run")
        .arg("-v")
        .arg(format!("{}:/host",
                     current_dir()
                         .expect("failed to get cwd")
                         .to_str()
                         .expect("bad utf-8 in cwd")))
        .arg("--name")
        .arg(&temp_name)
        .arg("-it")
        .arg(&name)
        .arg("/bin/bash")
        .spawn()
        .expect("Docker failed to launch")
        .wait()
        .expect("Docker failed at runtime");

    println!("Committing container history...");
    let _ = Command::new("docker")
        .arg("commit")
        .arg(&temp_name)
        .arg(&name_str)
        .output()
        .expect("Failed to commit docker changes");

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
    image_id: String,
    original_dockerfile: String, // TODO: not a string?
    original_workingpath: String, // TODO: not a string?
    pide_version: String, // TODO: not a string?

    // Tables go last
    name: PideName,
}

impl PideFile {
    pub fn to_string(&self) -> Result<String, ()> {
        match toml::to_string(self) {
            Ok(out) => Ok(out),
            Err(x) => {
                println!("bad: {:?}", x);
                Err(())
            }
        }
    }

    pub fn from_file(open_path: &str) -> Self {

        let mut file = File::open(open_path)
            .expect(&format!("Failed to open pide file {:}", open_path));
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        toml::from_str(&contents).expect("yiss")
    }
}

fn init(dockerfile: &str) {
    // println!("{}", dockerfile);

    // println!("{:?}", PideName::new().to_string());
    let dfp = Path::new(dockerfile);
    let working_path = dfp.parent().expect("Failed to choose working path");

    assert!(dfp.is_file(), "Dockerfile doesn't exist");
    assert!(working_path.exists(), "Working path doesn't exist");

    let build_cmd = Command::new("docker")
        .arg("build")
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

    let pide_data = PideFile {
        name: PideName::new(),
        image_id: image_id,
        original_dockerfile: dockerfile.to_string(),
        original_workingpath: working_path.to_str().unwrap().to_string(),
        pide_version: "0.2.0".to_string(),
    };

    println!("{:?}", pide_data);

    let open_path = ".pide";

    let mut file = File::create(open_path)
        .expect(&format!("Failed to open pide file {:}", open_path));
    let _ = file.write_all(&pide_data.to_string().expect("sadfoo").into_bytes())
        .expect("Failed to write to pide file");
}
