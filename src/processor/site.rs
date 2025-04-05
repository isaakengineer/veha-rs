use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::{motor, processor};

fn read_map(file_path: PathBuf) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true) // Skip the header
        .comment(Some(b'#')) // Ignore lines starting with '#'
        .from_reader(file);

    let mut data = Vec::new();
    for result in rdr.records() {
        let record = result?;
        if record.len() == 2 {
            data.push((record[0].to_string(), record[1].to_string()));
        }
    }
    Ok(data)
}

pub fn process(template_path: PathBuf, map_path: PathBuf, language: Option<&String>) {
    let data = read_map(map_path).unwrap();
    for row in data {
        // let input_path = Path::new(&row.0).to_path_buf();
        let input_path = template_path.join(&row.0);
        // let output_path = Path::new(&row.1).to_path_buf();
        let output_path = template_path.join(&row.1);

        match processor::preprocess::collection_name(output_path.clone()) {
            Some(name) => {
                let mut file = match std::fs::read_to_string(&input_path) {
                    Ok(content) => content,
                    Err(error) => {
                        log::error!("Failed to read file at path: {:?}", input_path);
                        panic!("The path provided via CLI could not be read!");
                    }
                };
                log::info!("Collection detected. Preprocessor activated.");
                motor::sqlite::lesen(file, template_path.clone(), language);
            }
            None => {
                processor::page::process(template_path.clone(), input_path, output_path, language);
            } // let collection = processor::collection::gen_context();
              // for context in collection {
              //     process::collection::gen_item()
              // }
        }
    }
}
