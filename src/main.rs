mod beide;
mod motor;
mod qwen;
mod schreiben;

use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use toml;

use beide::probe;
use motor::vorlage;
use qwen::transform;
use schreiben::beispiel_person;

#[derive(Parser)]
struct Cli {
    input_path: std::path::PathBuf,
    template_path: std::path::PathBuf,
}

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
    //  let beispiel = beispiel_person();
    //  println!("{:}", String::from_utf8(beispiel).unwrap());

    // probe();

    let args = Cli::parse();
    let mut file = std::fs::read_to_string(&args.input_path)
        .expect("The path provieded via CLI could not be read!");

    vorlage(file, args.template_path);
    println!("path: {:?}", args.input_path);

    // let toml_str =
    //     fs::read_to_string("beispiel/konfig.toml").expect("Failed to read Cargo.toml file");

    // let cargo_toml: CargoToml =
    //     toml::from_str(&toml_str).expect("Failed to deserialize Cargo.toml");

    // println!("{:#?}", cargo_toml);

    // transform();
}
