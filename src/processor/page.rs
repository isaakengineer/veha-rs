use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::motor;

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

pub fn process(
    template_path: PathBuf,
    input_path: PathBuf,
    output_path: PathBuf,
    language: Option<&String>,
) {
    let output_path = extend_extension_with_language(&output_path, language);

    let mut file = match std::fs::read_to_string(&input_path) {
        Ok(content) => content,
        Err(error) => {
            log::error!("Failed to read file at path: {:?}", input_path);
            panic!("The path provided via CLI could not be read!");
        }
    };

    let mut dateien = motor::sql::binden(file, template_path.clone(), language)
        .expect("something went wrong");

    // Ensure all parent directories exist
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create directories");
    }

    fs::write(output_path.clone(), &dateien).expect("msg");

    file = std::fs::read_to_string(output_path.clone()).expect("err");

    dateien = motor::csv::csv_tag_einfuellen(file, template_path.clone(), language)
        .expect("something went wrong");

    fs::write(output_path.clone(), &dateien).expect("msg");

    file = std::fs::read_to_string(output_path.clone()).expect("err");

    dateien = motor::xml::transform(file, template_path.clone()).expect("msg");

    fs::write(output_path.clone(), &dateien).expect("msg");

    file = std::fs::read_to_string(output_path.clone()).expect("err");

    dateien = motor::toml::transform(file, &template_path.as_path(), language)
        .expect("something went wrong").into();

    fs::write(output_path.clone(), &dateien).expect("msg");

    file = std::fs::read_to_string(output_path.clone()).expect("err");

    // let mut dateien = motor::csv::werte_ersetzen(file).expect("etwas schiefgelaufen");
    // let mut dateien = motor::csv::csv_tag_einfuellen(file, template_path).expect("error!");
    dateien = motor::md::werte_ersetzen(file, template_path.clone(), language)
        .expect("something went wrong");

    fs::write(output_path.clone(), &dateien).expect("msg");
}
