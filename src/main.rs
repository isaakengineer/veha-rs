mod beide;
mod csvpilot;
mod mdmotor;
mod motor;
mod qwen;
mod schreiben;
mod tomlmotor;
mod utils;

use clap::builder::{PossibleValuesParser, TypedValueParser};
use clap::{self, builder::PossibleValue};
use clap::{value_parser, Arg};
use csv;
use log::{debug, error, info, warn};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use toml;

use beide::probe;
use csvpilot::{csv_tag_einfuellen, reihe_einfuellen, werte_ersetzen};
use motor::vorlage;
use qwen::transform;
use schreiben::beispiel_person;

// #[derive(Parser)]
// struct Cli {
//     input_path: std::path::PathBuf,
//     template_path: std::path::PathBuf,
//     output_path: std::path::PathBuf,
// }

#[derive(Debug, Deserialize)]
struct Konfiguration {
    #[allow(dead_code)]
    vorlagen: Vec<Vorlage>,
}

#[derive(Debug, Deserialize)]
struct Vorlage {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    path: String,
}

fn extend_extension_with_language(path: &PathBuf, language: Option<&String>) -> PathBuf {
    let mut new_path = path.clone();
    if let Some(lang) = language {
        if let Some(extension) = path.extension() {
            let mut new_extension = std::ffi::OsString::new();
            new_extension.push(lang);
            new_extension.push(".");
            new_extension.push(extension);
            new_path.set_extension(new_extension);
        }
    }
    new_path
}

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

fn process_page(
    template_path: PathBuf,
    input_path: PathBuf,
    output_path: PathBuf,
    language: Option<&String>,
) {
    let output_path = extend_extension_with_language(&output_path, language);

    let mut file = std::fs::read_to_string(&input_path)
        .expect("The path provieded via CLI could not be read!");

    // let mut dateien = werte_ersetzen(file).expect("etwas schiefgelaufen");
    // let mut dateien = csv_tag_einfuellen(file, template_path).expect("error!");
    let mut dateien = mdmotor::werte_ersetzen(file, template_path.clone(), language)
        .expect("something went wrong");

    fs::write(output_path.clone(), &dateien).expect("msg");

    file = std::fs::read_to_string(output_path.clone()).expect("err");

    let mut dateien =
        csv_tag_einfuellen(file, template_path.clone(), language).expect("something went wrong");

    fs::write(output_path.clone(), &dateien).expect("msg");

    file = std::fs::read_to_string(output_path.clone()).expect("err");

    dateien = transform(file, template_path.clone()).expect("msg");

    fs::write(output_path.clone(), &dateien).expect("msg");

    file = std::fs::read_to_string(output_path.clone()).expect("err");

    let mut dateien =
        tomlmotor::motor(file, &template_path.as_path(), language).expect("something went wrong");

    fs::write(output_path.clone(), &dateien).expect("msg");
}

fn process_site(template_path: PathBuf, map_path: PathBuf, language: Option<&String>) {
    let data = read_map(map_path).unwrap();
    for row in data {
        let input_path = Path::new(&row.0).to_path_buf();
        let output_path = Path::new(&row.1).to_path_buf();
        process_page(template_path.clone(), input_path, output_path, language);
    }
}

fn main() {
    env_logger::init();

    //  let beispiel = beispiel_person();
    //  println!("{:}", String::from_utf8(beispiel).unwrap());

    // probe();

    // let args = Cli::parse();
    // println!("path: {:?}", input_path);

    let matches = clap::Command::new("leva")
        .subcommand(
            clap::Command::new("page")
                .arg(
                    Arg::new("template")
                        .required(true)
                        .index(1)
                        .help("Template path"),
                )
                .arg(Arg::new("input").required(true).index(2).help("Input path"))
                .arg(
                    Arg::new("output")
                        .required(true)
                        .index(3)
                        .help("Output path"),
                )
                .arg(
                    Arg::new("language")
                        .help("The language code for the content and webpage")
                        .short('l')
                        .long("language")
                        .value_parser(clap::builder::PossibleValuesParser::new(&[
                            "de", "en", "fr",
                        ])),
                ),
        )
        .subcommand(
            clap::Command::new("site")
                .arg(
                    Arg::new("template")
                        .required(true)
                        .index(1)
                        .help("Template path"),
                )
                .arg(Arg::new("map").required(true).index(2).help("Input path"))
                .arg(
                    Arg::new("language")
                        .help("The language code for the content and webpage")
                        .short('l')
                        .long("language")
                        .value_parser(clap::builder::PossibleValuesParser::new(&[
                            "de", "en", "fr",
                        ])),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("page", sub_m)) => {
            let input_path = Path::new(sub_m.get_one::<String>("input").unwrap()).to_path_buf();
            let template_path =
                Path::new(sub_m.get_one::<String>("template").unwrap()).to_path_buf();
            let mut output_path =
                Path::new(sub_m.get_one::<String>("output").unwrap()).to_path_buf();
            let language = sub_m.get_one::<String>("language");

            // Use the arguments in your code
            if let Some(lang) = language {
                info!("Language: {}", lang);
            } else {
                info!("No language code provided, default will be used.");
            }

            // Process the "page" subcommand with the provided arguments
            process_page(template_path, input_path, output_path, language);
        }
        Some(("site", sub_m)) => {
            let template_path =
                Path::new(sub_m.get_one::<String>("template").unwrap()).to_path_buf();
            let map_path = Path::new(sub_m.get_one::<String>("map").unwrap()).to_path_buf();
            let language = sub_m.get_one::<String>("language");

            // Use the arguments in your code
            if let Some(lang) = language {
                info!("Language: {}", lang);
            } else {
                info!("No language code provided, default will be used.");
            }

            // Process the "site" subcommand with the provided arguments
            process_site(template_path, map_path, language);
        }
        _ => {
            eprintln!("No valid subcommand was provided.");
        }
    }

    // println!("Input Path: {}", input_path);
    // println!("Template Path: {}", template_path);
    // println!("Output Path: {}", output_path);

    // vorlage(file, template_path);

    // let toml_str =
    //     fs::read_to_string("beispiel/konfig.toml").expect("Failed to read Cargo.toml file");

    // let cargo_toml: CargoToml =
    //     toml::from_str(&toml_str).expect("Failed to deserialize Cargo.toml");

    // println!("{:#?}", cargo_toml);

    // transform();
}
