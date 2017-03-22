use std::io::prelude::*;
use std::fs::File;
use uuid::Uuid;
use toml;


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
    pub image_id: String,
    pub original_dockerfile: String, // TODO: not a string?
    pub original_workingpath: String, // TODO: not a string?
    pub pide_version: String, // TODO: not a string?

    // Tables go last
    pub name: PideName,
}

impl PideFile {
    pub fn to_string(&self) -> Result<String, ()> {
        match toml::to_string(self) {
            Ok(out) => Ok(out),
            Err(x) => {
                println!("Failed to serialize pidefile: {:?}", x);
                Err(())
            }
        }
    }

    pub fn to_file(&self, path: &str) {
        let mut file = File::create(path).expect(&format!("Failed to open pide file {:}", path));
        let _ = file.write_all(&self.to_string().expect("sadfoo").into_bytes())
            .expect("Failed to write to pide file");
    }

    pub fn from_file(open_path: &str) -> Self {

        let mut file = File::open(open_path)
            .expect(&format!("Failed to open pide file {:}", open_path));
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        toml::from_str(&contents).expect("yiss")
    }
}
