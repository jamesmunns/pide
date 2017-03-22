use std::path::Path;

use clap::{App, SubCommand, Arg};
use uuid::Uuid;

use docker;
use types::{PideFile, PideName};


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

        // "pide ssh"
        .subcommand(SubCommand::with_name("ssh")
            .about("Attach to a running pide"))

        .get_matches();

    match matches.subcommand() {
        ("init", Some(args)) => {
            let dockerfile = args.value_of("Dockerfile").expect("Missing required argument?");
            init(dockerfile);
        }
        ("resume", _) => {
            resume();
        }
        ("ssh", _) => {
            ssh();
        }
        _ => println!("Hallo, welt!"),
    }
}

// Okay, its not really ssh. But it acts a lot like it
fn ssh() {
    if let Some(id) = docker::get_image_name(&PideFile::from_file(".pide").name) {
        println!("Joining container in progress...");
        docker::ssh_attach(&id);
    } else {
        println!("Container not running. Try `pide resume` first.");
    }
}

fn resume() {
    // Load pide file
    let pide_data = PideFile::from_file(".pide");
    assert_eq!(pide_data.pide_version, "0.2.0", "Version Mismatch!");

    let dockerfile = pide_data.original_dockerfile;
    let name_str = pide_data.name.to_string();
    let temp_name = Uuid::new_v4().to_string();

    println!("Resuming `{}`...", &dockerfile);
    docker::run(&temp_name, &name_str);

    println!("Committing container history...");
    docker::commit(&temp_name, &name_str);
}


fn init(dockerfile: &str) {
    let dfp = Path::new(dockerfile);
    let working_path = dfp.parent().expect("Failed to choose working path");

    assert!(dfp.is_file(), "Dockerfile doesn't exist");
    assert!(working_path.exists(), "Working path doesn't exist");

    let pide_name = PideName::new();

    let image_id = docker::build(&pide_name.to_string(), dockerfile, working_path);

    let pide_data = PideFile {
        name: pide_name,
        image_id: image_id,
        original_dockerfile: dockerfile.to_string(),
        original_workingpath: working_path.to_str().unwrap().to_string(),
        pide_version: "0.2.0".to_string(),
    };

    pide_data.to_file(".pide");

}
