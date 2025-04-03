use quick_xml::events::{BytesStart, Event};
use quick_xml::Writer;
use quick_xml::{Decoder, Reader};
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{Cursor, Read};
use std::path::Path;
use std::path::PathBuf;
use tera::{Context, Tera};
use toml::Value;

use crate::utils;

/// Processes an XHTML file with `<tera />` tags
pub fn motor(
    input_xhtml: String,
    base_dir: &Path,
    language: Option<&String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_str(&input_xhtml);
    // reader.trim_text(true);
    let mut output_buffer: Vec<u8> = Vec::new();
    let mut writer = Writer::new(&mut output_buffer);

    let mut config_path_fin: Option<PathBuf> = None;
    let mut config_name: Option<String> = None;
    let mut tera_pfahne = false;
    let mut contexts: Vec<(Option<String>, Option<PathBuf>)> = Vec::new();

    // println!("input string: {:?}", input_xhtml.clone());
    println!("parsing TOML config");
    // let mut buf = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) if e.local_name().as_ref() == b"toml" => {
                // Parse the `<tera />` tag attributes
                let attrs = parse_tag_attrs(&e)?;

                if let (Some(src)) = (attrs.get("src")) {
                    let mut config_path = base_dir.join(src);
                    if utils::attribut_vorhanden(e, "multilingual") {
                        config_path = utils::endung_mit_sprache_erweitern(&config_path, language);
                    }
                    println!("TOML file path: {:?}", config_path);
                    config_path_fin = Some(config_path);
                    config_name = attrs.get("name").map(|s| s.to_owned());
                    tera_pfahne = true;
                    contexts.push((config_name, config_path_fin));
                } else {
                    let error: Box<dyn Error> =
                        "no src attribute is supplied for the toml tag.".into();
                    return Err(error);
                }
            }
            Ok(Event::End(e)) if e.local_name().as_ref() == b"toml" => {}
            Ok(Event::Eof) => break,
            Ok(e) => {
                // Write all other events as-is
                writer.write_event(e)?;
            }
            Err(e) => return Err(e.into()),
            _ => {}
        }
    }
    // let result = writer.into_inner();
    let string_result = String::from_utf8(output_buffer)?;
    let mut final_result = string_result.clone();
    if tera_pfahne {
        final_result = process_via_tera(contexts, string_result).unwrap();
    }
    Ok(final_result)
}

fn process_via_tera(
    contexts: Vec<(Option<String>, Option<PathBuf>)>,
    input_string: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut result = "".to_string();
    let mut tera = Tera::default();
    let mut context = tera::Context::new();
    for c in contexts {
        let config_name = c.0;
        let config_path = c.1;
        if let Some(c_path) = config_path {
            println!("{:?}", c_path.clone());
            let toml_content = fs::read_to_string(c_path).expect("Failed to read config.toml");
            let config: Value = toml::from_str(&toml_content).expect("Failed to parse config.toml");

            tera.add_raw_template("template.xhtml", &input_string)
                .expect("Failed to add template to Tera");
            if let Some(c_name) = config_name {
                // Create a context with the parsed TOML data
                context.insert(c_name.as_str(), &config);
            } else {
                // Create a context with the parsed TOML data
                let mut context = tera::Context::new();
                context.insert("config", &config);
            }
        }
    }

    let rendered = tera
        .render("template.xhtml", &context)
        .expect("Failed to render template");
    result = rendered;
    Ok(result)
}

/// Parses attributes from a `<toml />` tag
fn parse_tag_attrs(e: &BytesStart) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut attrs = HashMap::new();
    for attr in e.attributes() {
        let attr = attr.unwrap();
        let local_name = attr.key.local_name();
        let key = String::from_utf8_lossy(local_name.as_ref());
        let value = attr.unescape_value().unwrap();
        let v: String = match value {
            Cow::Borrowed(borrowed) => borrowed.to_string().clone(),
            Cow::Owned(owned) => owned,
        };
        let k: String = match key {
            Cow::Borrowed(borrowed) => borrowed.to_string(),
            Cow::Owned(owned) => owned,
        };
        attrs.insert(k, v);
    }
    Ok(attrs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_no_toml_tag() {
        let xhtml_input = r#"
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
    <title>Test Document</title>
</head>
<body>
	<h1>Hello Wolrd!</h1>
</body>
</html>
        "#;

        let toml_content = r#"
title = "Dynamic Title"
description = "This content comes from TOML!"
        "#;

        // Create test config
        let test_dir = Path::new("test_data");
        fs::create_dir_all(test_dir).unwrap();
        fs::write(test_dir.join("test_config.toml"), toml_content).unwrap();

        let result_empty = motor(xhtml_input.to_string(), test_dir).unwrap();

        assert!(result_empty.contains("<h1>Hello Wolrd!</h1>"));

        // Cleanup
        fs::remove_dir_all(test_dir).unwrap();
    }

    #[test]
    fn test_process_via_tera() {
        let xhtml_input = r#"
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
    <title>Test Document</title>
    <toml src="test_config.toml" name="conf"></toml>
</head>
<body>
    <h1>{{ conf.title }}</h1>
    <p>{{ conf.description }}</p>
</body>
</html>
        "#;

        let toml_content = r#"
title = "Dynamic Title"
description = "This content comes from TOML!"
        "#;

        // Create test config
        let test_dir = Path::new("test_data_2");
        fs::create_dir_all(test_dir).unwrap();
        fs::write(test_dir.join("test_config.toml"), toml_content).unwrap();

        let result = motor(xhtml_input.to_string(), test_dir).unwrap();

        assert!(result.contains("<h1>Dynamic Title</h1>"));
        assert!(result.contains("<p>This content comes from TOML!</p>"));
        assert!(!result.contains("<toml")); // Ensure the <tera /> tag is removed

        // Cleanup
        fs::remove_dir_all(test_dir).unwrap();
    }
}
