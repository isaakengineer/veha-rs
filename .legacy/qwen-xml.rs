use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use xml::reader::{EventReader, XmlEvent};
use xml::writer::{EmitterConfig, EventWriter};

pub fn transform() -> Result<(), Box<dyn std::error::Error>> {
    let buch_xml_path = Path::new("buch/eintrag.xhtml");
    let vorlagen_dir = Path::new("vorlagen");

    // Read the content of buch/eintrag.xhtml
    let mut file = fs::File::open(buch_xml_path)?;
    let reader = BufReader::new(file);
    let events = EventReader::from_reader(reader);

    // Create a vector to hold the processed XML data
    let mut output_buffer: Vec<u8> = Vec::new();
    let emitter = EmitterConfig::create(&mut output_buffer).write_document_declaration(false);
    let mut writer = EventWriter::new(emitter);

    for event in events {
        match event? {
            XmlEvent::StartElement { name, attributes, .. } if name.local_name == "vorlage" => {
                // Extract the value of the 'name' attribute
                if let Some(name_attr) = attributes.iter().find(|attr| attr.name.local_name == "name") {
                    let template_name = &name_attr.value;

                    // Construct the path to the template file
                    let template_path = vorlagen_dir.join(template_name).with_extension("xhtml");
                    if template_path.exists() {
                        // Read and process the template file
                        let template_content = fs::read_to_string(&template_path)?;

                        // Replace <slot></slot> with content from the original file
                        let mut slot_replaced = false;
                        let mut template_reader = BufReader::new(template_content.as_bytes());
                        let template_events = EventReader::from_reader(template_reader);

                        for event in template_events {
                            match event? {
                                XmlEvent::StartElement { name, .. } if name.local_name == "slot" => {
                                    // Replace the <slot></slot> with content from the original file
                                    writer.write_characters(&format!("<{}>{}</{}>", name.local_name, attributes_to_string(&attributes), "</{}>",
name.local_name))?;
                                    slot_replaced = true;
                                }
                                XmlEvent::EndElement { .. } if !slot_replaced => {
                                    // Write everything else from the template file as is
                                    writer.write_event(event)?;
                                }
                                _ => {
                                    // Write everything else from the template file as is
                                    writer.write_event(event)?;
                                }
                            }
                        }
                    } else {
                        eprintln!("Template file not found: {:?}", template_path);
                    }
                }
            }
            _ => {
                writer.write_event(event)?;
            }
        }
    }

    // Write the processed XML to a new file or standard output
    fs::write("output.xhtml", &output_buffer)?;

    Ok(())
}

fn attributes_to_string(attributes: &[xml::reader::Attribute]) -> String {
    attributes.iter()
               .map(|attr| format!(r#" {}="{}""#, attr.name.local_name, attr.value))
               .collect::<Vec<_>>()
               .join("")
}
