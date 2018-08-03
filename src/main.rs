extern crate clap;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate reqwest;
extern crate failure;
extern crate regex;


use regex::Regex;
use failure::Error;
use clap::{Arg, App};
use std::io::Read;
use std::fs::File;


fn get_sha(path: &str) -> Result<String, Error> {
    let mut resp = reqwest::get(&format!("{}.sha256", path))?;
    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content)?;

    Ok(String::from(content.split_whitespace().next().unwrap()))
}


fn main() {
        let matches = App::new("Update flatpak json")
                          .version("1.0")
                          .author("Daniel García <danigm@wadobo.com>")
                          .about("Updates the flatpak .json file with new rust stable download")
                          .arg(Arg::with_name("JSON")
                               .help("the json file to update")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("CURRENT_VERSION")
                               .help("the rust version in the file")
                               .required(true)
                               .index(2))
                          .arg(Arg::with_name("NEXT_VERSION")
                               .help("the new rust version")
                               .required(true)
                               .index(3))
                          .get_matches();

        let json_file = matches.value_of("JSON").unwrap();
        let from_v = matches.value_of("CURRENT_VERSION").unwrap();
        let to_v = matches.value_of("NEXT_VERSION").unwrap();

        let re = Regex::new(&format!("{}", from_v).replace('.', "\\.")).unwrap();

        eprintln!("Updating {} from {} to {}", json_file, from_v, to_v);

        let mut file = File::open(json_file).unwrap();
        let mut serialized = String::new();
        file.read_to_string(&mut serialized).unwrap();

        let mut json: serde_json::Value = serde_json::from_str(&serialized).unwrap();

        {
            let modules = json["modules"].as_array_mut().unwrap();
            for m in modules {
                if m["name"] != "rust" {
                    continue;
                }

                let sources = m["sources"].as_array_mut().unwrap();
                for s in sources {
                    let url = String::from(s["url"].as_str().unwrap());
                    eprintln!("URL: {}", &url);
                    let newurl = re.replace_all(&url, to_v);
                    eprintln!("new URL: {}", newurl);
                    s["sha256"] = json!(get_sha(&newurl).unwrap());
                    s["url"] = json!(newurl);
                }
            }
        }

        let output = serde_json::to_string_pretty(&json).unwrap();
        println!("{}", output);
}
