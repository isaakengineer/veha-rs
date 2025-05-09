mod motor {
    pub mod csv;
    pub mod md;
    pub mod sqlite;
    pub mod toml;
    pub mod xml;
}

mod processor {
    pub mod collection;
    pub mod page;
    pub mod site;
}

mod machine {
    pub mod binder;
    pub mod press;
}

pub mod utils;

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

fn main() {
    env_logger::init();

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
            processor::page::process(template_path, input_path, output_path, language);
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
            processor::site::process(template_path, map_path, language);
        }
        _ => {
            eprintln!("No valid subcommand was provided.");
        }
    }

    // println!("Input Path: {}", input_path);
    // println!("Template Path: {}", template_path);
    // println!("Output Path: {}", output_path);

    // xml::vorlage(file, template_path);

    // let toml_str =
    //     fs::read_to_string("beispiel/konfig.toml").expect("Failed to read Cargo.toml file");

    // let cargo_toml: CargoToml =
    //     toml::from_str(&toml_str).expect("Failed to deserialize Cargo.toml");

    // println!("{:#?}", cargo_toml);

    // motor::xml::transform();
}
