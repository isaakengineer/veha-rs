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

/// Processes an XHTML file with `<tera />` tags
pub fn process_via_tera(
    input_xhtml: String,
    base_dir: &Path,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_str(&input_xhtml);
    // reader.trim_text(true);
    let mut output_buffer: Vec<u8> = Vec::new();
    let mut writer = Writer::new(&mut output_buffer);
    let mut tera = Tera::default();
    let mut context = Context::new();
    let mut config_path: Option<PathBuf> = None;
    let mut config_name: Option<String> = None;

    // println!("input string: {:?}", input_xhtml.clone());

    // let mut buf = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) if e.local_name().as_ref() == b"tera" => {
                println!("tera tag found!");
                // Parse the `<tera />` tag attributes
                let attrs = parse_tag_attrs(&e)?;
                if let (Some(src)) = (attrs.get("src")) {
                    // Load the TOML file
                    config_path = Some(base_dir.join(src));
                    config_name = attrs.get("name").map(|s| s.to_owned());
                } else {
                    let error: Box<dyn Error> =
                        "no src attribute is supplied for the tera tag.".into();
                    return Err(error);
                }
            }
            Ok(Event::End(e)) if e.local_name().as_ref() == b"tera" => {
                println!("tera end tag found!");
            }
            Ok(Event::Eof) => break,
            Ok(e) => {
                println!("Event is: {:?}", e);
                // Write all other events as-is
                writer.write_event(e)?;
            }
            Err(e) => return Err(e.into()),
            _ => {}
        }
    }
    // let result = writer.into_inner();
    let string_result = String::from_utf8(output_buffer)?;
    println!("Final result: {:?}", string_result.clone());
    let mut final_result = "".to_string();
    if let Some(c_path) = config_path {
        let toml_content = fs::read_to_string(c_path).expect("Failed to read config.toml");
        let config: Value = toml::from_str(&toml_content).expect("Failed to parse config.toml");
        let mut tera = Tera::default();
        tera.add_raw_template("template.xhtml", &string_result)
            .expect("Failed to add template to Tera");
        println!("config name is {:?}", config_name.clone());
        if let Some(c_name) = config_name {
            println!("name is supplied!");
            // Create a context with the parsed TOML data
            let mut context = tera::Context::new();
            context.insert(c_name.as_str(), &config);
            let rendered = tera
                .render("template.xhtml", &context)
                .expect("Failed to render template");
            final_result = rendered;
        } else {
            // Create a context with the parsed TOML data
            let mut context = tera::Context::new();
            context.insert("config", &config);
            let rendered = tera
                .render("template.xhtml", &context)
                .expect("Failed to render template");
            final_result = rendered;
        }
    }
    println!("Final result: {:?}", final_result.clone());
    Ok(final_result)
}

/// Parses attributes from a `<tera />` tag
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
    fn test_process_via_tera() {
        let xhtml_input = r#"
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
    <title>Test Document</title>
    <tera src="test_config.toml" name="conf"></tera>
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
        let test_dir = Path::new("test_data");
        fs::create_dir_all(test_dir).unwrap();
        fs::write(test_dir.join("test_config.toml"), toml_content).unwrap();

        let result = process_via_tera(xhtml_input.to_string(), test_dir).unwrap();

        assert!(result.contains("<h1>Dynamic Title</h1>"));
        assert!(result.contains("<p>This content comes from TOML!</p>"));
        assert!(!result.contains("<tera")); // Ensure the <tera /> tag is removed

        // Cleanup
        fs::remove_dir_all(test_dir).unwrap();
    }
}
