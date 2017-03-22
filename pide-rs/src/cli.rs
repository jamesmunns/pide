use clap::{App, SubCommand, ArgMatches, Arg};
use uuid::Uuid;
use std::path::Path;
use std::process::Command;

use toml;
use std::io::prelude::*;
use std::fs::File;



pub fn parse_cli() {
    let matches = App::new("pide: Persistent Isolated Development Environments (in Docker)")
        .author(crate_authors!())
        .version(crate_version!())

        // "zeit init"
        .subcommand(SubCommand::with_name("init")
            .about("Initialize pide here")
            .arg(Arg::with_name("Dockerfile")
                .help("Dockerfile to initialize this environment with")
                .required(true)))

        // // "zeit pom"
        // .subcommand(SubCommand::with_name("pom")
        //     .about("Start, end, or modify a pomodoro time")

        //     // "zeit pom start"
        //     .subcommand(SubCommand::with_name("start")
        //         .help("start a new pomodoro session")))
        .get_matches();

    match matches.subcommand() {
        ("init", Some(args)) => {
            let dockerfile = args.value_of("Dockerfile").expect("Missing required argument?");
            init(dockerfile);
        }
        //     ("pom", Some(x)) => {
        //         pomodoro(x);
        //     }
        _ => println!("Hallo, welt!"),
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Pidename {
    repo: String,
    id: Uuid,
}

impl Pidename {
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
pub struct Pidefile {
    image_id: String,
    original_dockerfile: String, // TODO: not a string?
    original_workingpath: String, // TODO: not a string?
    pide_version: String, // TODO: not a string?

    // Tables go last
    name: Pidename,
}

impl Pidefile {
    pub fn to_string(&self) -> Result<String, ()> {
        match toml::to_string(self) {
            Ok(out) => Ok(out),
            Err(x) => {
                println!("bad: {:?}", x);
                Err(())
            }
        }
    }
}

fn init(dockerfile: &str) {
    // println!("{}", dockerfile);

    // println!("{:?}", Pidename::new().to_string());
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

    let pide_data = Pidefile {
        name: Pidename::new(),
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
