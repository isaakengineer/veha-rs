use std::collections::HashMap;
use std::fs;
use std::fs::{create_dir_all, File};
use std::io::Read;
use std::io::Write;
use std::path::{Path, PathBuf};
use log;

use crate::{machine, motor, processor};

const EXPANSION_SIGN: char = '+';
const COLLECTION_SIGN_START: char = '{';
const COLLECTION_SIGN_END: char = '}';

fn ausgabepfadersteller(path: PathBuf, params: HashMap<String, String>) -> PathBuf {
    let path_str = path.to_string_lossy().to_string();
    let mut result = path_str.clone();

    for (key, value) in params {
        let placeholder = format!("+{{{}}}", key);
        result = result.replace(&placeholder, &value);
    }

    log::info!("Ausgabepfad erstellt: {:?}", result);

    PathBuf::from(result)
}

fn write_hashmap_to_file(map: &HashMap<String, String>, file_path: PathBuf) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(map).expect("Failed to serialize HashMap");
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn write_vechashmap_to_file(
    map: &Vec<HashMap<String, String>>,
    file_path: &str,
) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(map).expect("Failed to serialize HashMap");
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn binden(
    template_path: PathBuf,
    eingabepfad: PathBuf,
    ausgabepfad: PathBuf,
    language: Option<&String>,
) {
    let mut eingabedatei: (Vec<u8>, Vec<u8>) = (Vec::new(), Vec::new());
    let mut map: Vec<HashMap<String, String>> = Vec::new();
    log::info!("binden mit {:?}, {:?}, {:?}", template_path, eingabepfad, ausgabepfad);
    match kennzeichen_ausgeben(ausgabepfad.clone()) {
        Some(name) => {
            let mut file = match std::fs::read_to_string(&eingabepfad) {
                Ok(content) => {
                    log::info!("File read successfully.");
                    eingabedatei = machine::press::verteilen_nach_tag(
                        &content.as_str(),
                        "backend".to_string().as_str(),
                    )
                    .unwrap();
                    log::info!("Collection detected. Preprocessor activated.");
                    // log::info!(
                    //     "Eingabedatei ist: {:?}, {:?}",
                    //     std::str::from_utf8(&eingabedatei.0.clone()),
                    //     std::str::from_utf8(&eingabedatei.0.clone()),
                    // );
                    map = motor::sqlite::gen_map(eingabedatei.0, template_path.clone(), language)
                        .unwrap();
                    for row in map {
                        let ausgabepfad_einzeln =
                            ausgabepfadersteller(ausgabepfad.clone(), row.clone());
                        // JSON test
                        // write_hashmap_to_file(&row, ausgabepfad_einzeln);

                        // log::info!(
                        //     "erstellen Augabe-Vector mit {:?},{:?},{:?}",
                        //     eingabedatei.1.clone(),
                        //     row.clone(),
                        //     b"w"
                        // );
                        let mut ausgabe_vec = machine::press::hashmap_drucken(
                            eingabedatei.1.clone(),
                            row.clone(),
                            b"w",
                        )
                        .unwrap();
                        if let Some(parent) = ausgabepfad_einzeln.parent() {
                            create_dir_all(parent).unwrap();
                        }
                        let mut file = File::create(ausgabepfad_einzeln.clone()).unwrap();
                        file.write_all(&ausgabe_vec).unwrap();
                        processor::page::process(
                            template_path.clone(),
                            ausgabepfad_einzeln.clone(),
                            ausgabepfad_einzeln.clone(),
                            language,
                        );
                    }
                }
                Err(error) => {
                    log::error!("Failed to read file at path: {:?}", eingabepfad);
                    panic!("The path provided via CLI could not be read!");
                }
            };
        }
        None => {
            processor::page::process(template_path.clone(), eingabepfad, ausgabepfad, language);
        } // let collection = processor::collection::gen_context();
          // for context in collection {
          //     process::collection::gen_item()
          // }
    }
}

pub fn kennzeichen_ausgeben(pfad: std::path::PathBuf) -> Option<String> {
	for component in pfad.components() {
        if let Some(stem) = component.as_os_str().to_str() {
            // Check if the stem contains the specific pattern
            if let Some(plus_pos) = stem.find('+') {
                if let Some(open_brace_pos) = stem.find('{') {
                    if let Some(close_brace_pos) = stem.find('}') {
                        // Ensure the positions are in the correct order
                        if plus_pos < open_brace_pos && open_brace_pos < close_brace_pos {
                            // Extract the content between `{` and `}`
                            let key = &stem[open_brace_pos + 1..close_brace_pos];
                            return Some(key.to_string());
                        }
                    }
                }
            }
        }
    }
    None

    // let dateiname = pfad.file_stem().and_then(|name| name.to_str());
    // log::info!("Dateiname = {:?}", dateiname.clone());
    // if let Some(dateiname) = dateiname {
    //     if let Some(start) = dateiname.find(COLLECTION_SIGN_START) {
    //         if let Some(ende) = dateiname.find(COLLECTION_SIGN_END) {
    //             if start < ende && dateiname.chars().nth(start - 1) == Some(EXPANSION_SIGN) {
    //                 let key = &dateiname[start + 1..ende];
    //                 return Some(key.to_string());
    //             }
    //         }
    //     }
    // }
    // return None;

    // let endung = pfad.extension().and_then(|ext| ext.to_str());
    // match endung {
    //     Some("xhtml") | Some("json") => {
    //         // Check for the pattern in the file name
    //     }
    //     None => (false),
    // }
}

pub fn process() {}
