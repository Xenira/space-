use protocol_types::heros::God;
use serde::{Deserialize, de::DeserializeOwned};
use serde_json;
use std::{fs::File, io::BufReader};

trait ToString {
    fn to_string(&self, idx: usize) -> String;
}

#[derive(Deserialize)]
struct GodJson {
    name: String,
    description: String,
    pantheon: String,
}

impl ToString for GodJson {
    fn to_string(&self, idx: usize) -> String {
        format!(
            "God {{ id: {}, name: \"{}\".to_string(), description: \"{}\".to_string(), pantheon: \"{}\".to_string() }}",
            idx,
            self.name,
            self.description,
            self.pantheon
        )
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=./gods.json");

    generate_from_json::<GodJson, God>("./data/gods.json", "gods")?;

    Ok(())
}

fn generate_from_json<T, U>(filename: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> where T: ToString + DeserializeOwned {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let gods_rs_path = std::path::Path::new(&out_dir).join(format!("{}.rs", name));
    let type_name = std::any::type_name::<U>();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let entries: Vec<T> = serde_json::from_reader(reader)?;
    let mut generated_rs = vec![
        "use static_init::dynamic;".to_string(),
        format!("use {};", type_name),
    ];

    generated_rs.push(format!(
        "#[dynamic]\npub static {}: Vec<{}> = vec![{}];", name.to_uppercase(), type_name,
        entries.iter().enumerate().map(|(idx, item)| item.to_string(idx)).collect::<Vec<_>>()
            .join(", ")
    ));

    std::fs::write(&gods_rs_path, generated_rs.join("\n")).unwrap();

    Ok(())
}