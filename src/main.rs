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
use log::{debug, error, info, warn};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
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

fn main() {
    env_logger::init();

    //  let beispiel = beispiel_person();
    //  println!("{:}", String::from_utf8(beispiel).unwrap());

    // probe();

    // let args = Cli::parse();
    // println!("path: {:?}", input_path);

    let matches = clap::Command::new("leva")
        .arg(Arg::new("input").required(true).index(1).help("Input path"))
        .arg(
            Arg::new("template")
                .required(true)
                .index(2)
                .help("Template path"),
        )
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
        )
        .get_matches();

    let input_path = Path::new(matches.get_one::<String>("input").unwrap()).to_path_buf();
    let template_path = Path::new(matches.get_one::<String>("template").unwrap()).to_path_buf();
    let mut output_path = Path::new(matches.get_one::<String>("output").unwrap()).to_path_buf();
    let language = matches.get_one::<String>("language");

    output_path = extend_extension_with_language(&output_path, language);
    // Use the arguments in your code
    if let Some(lang) = language {
        info!("Language: {}", lang);
    }
    info!("No language code provided, default will be used.");

    // println!("Input Path: {}", input_path);
    // println!("Template Path: {}", template_path);
    // println!("Output Path: {}", output_path);

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
    // vorlage(file, template_path);

    // let toml_str =
    //     fs::read_to_string("beispiel/konfig.toml").expect("Failed to read Cargo.toml file");

    // let cargo_toml: CargoToml =
    //     toml::from_str(&toml_str).expect("Failed to deserialize Cargo.toml");

    // println!("{:#?}", cargo_toml);

    // transform();
}
