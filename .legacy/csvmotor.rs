use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::borrow::Cow;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn einsetzen(
    file: String,
    vorlagen_dir: std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // let mut reader = BufReader::new(file);
    let mut xml_reader = Reader::from_str(&file);
    xml_reader.config_mut().trim_text(true);

    // Create a vector to hold the processed XML data
    let mut output_buffer: Vec<u8> = Vec::new();
    let mut writer = Writer::new(&mut output_buffer);

    let mut csv_fahne_anfang = false;
    let mut csv_fahne_ende = false;
    let mut csv_datei_pfad;
    let mut rdr;

    // for event in xml_reader.read_event() {
    // match event? {
    loop {
        match xml_reader.read_event() {
            Ok(Event::Start(csv)) if csv.local_name().as_ref() == b"csv" => {
                csv_fahne = true;

                // Extract the value of the 'name' attribute
                if let Some(src_attr) = csv
                    .attributes()
                    .find(|attr| attr.clone().unwrap().key.local_name().as_ref() == b"src")
                {
                    let src_name = src_attr.unwrap().value;

                    let name: String = match src_name {
                        Cow::Borrowed(borrowed) => String::from_utf8(borrowed.to_vec()).unwrap(),
                        Cow::Owned(owned) => String::from_utf8(owned).unwrap(),
                    };

                    // Construct the path to the template file
                    let csv_datei_pfad = vorlagen_dir.join(name).with_extension("csv");

                    println!("CSV-Datei wird hervorgerufen");
                }
            },
            Ok(Event::Start(row)) if row.name().as_ref() == b"row" => {
            	// TODO: check if the csv file already was flagged and is being looked for!
                if csv_datei_pfad.exists() {
                    // Read and process the template file
                    let csv_content = fs::read_to_string(&csv_datei_pfad)?;

                    rdr = csv::Reader::from_reader(csv_content);

                    for result in rdr.deserialize() {
                        let record: Record = result?;
                        println!("{:?}", record);
                    }

                        // Replace <slot></slot> with content from the original file

                        // let mut template_reader = BufReader::new(template_content.as_bytes());
                        let mut template_xml_reader = Reader::from_str(&template_content);
                        template_xml_reader.config_mut().trim_text(true);
                        println!("vorlage 2");
                        // for event in events {
                        // match event? {
                        let mut end_found = false;
                        loop {
                            match template_xml_reader.read_event() {
                                Ok(Event::Start(slot)) if slot.name().as_ref() == b"slot" => {
                                    println!("match 1");
                                    // Write the <slot> tag
                                    // writer.write_event(Event::Start(start.clone()))?;

                                    // Read and write everything inside <slot>
                                    let mut xml_reader_clone = xml_reader.clone();
                                    loop {
                                        match xml_reader_clone.read_event() {
                                            Err(e) => return Err(e.into()),
                                            Ok(Event::Eof) => {
                                                // break
                                                return Err("Premature EOF".into());
                                                // Notiz: es ist immer eine falsche EOF, da man tatsächlich eine Endetag für `</vorlage>` braucht.
                                            }
                                            Ok(Event::End(event)) => {
                                                if event.name() == vorlage.name() {
                                                    end_found = true;
                                                    break;
                                                } else {
                                                    // let e = Event::new(event.name());
                                                    writer.write_event(Event::End(event))?;
                                                }
                                            }
                                            Ok(event) => {
                                                writer.write_event(event)?;
                                            }
                                        }
                                    }
                                    // Write the </slot> tag
                                    if end_found {
                                        // writer.write_event(Event::End(BytesEnd::new("slot")))?;
                                    } else {
                                        return Err("No corresponding </slot> found".into());
                                    }
                                    slot_replaced = true;
                                }
                                Ok(Event::Eof) => break,
                                Ok(Event::End(slot)) if slot.name().as_ref() == b"slot" => {
                                    // DO NOTHING
                                }
                                Ok(event) => {
                                    println!("match 2");
                                    // write everything else from template file as is
                                    writer.write_event(event)?;
                                }
                                Err(e) => return Err(e.into()),
                            }
                        }
                    } else {
                        eprintln!("Template file not found: {:?}", template_path);
                    }
                }
            }
            Ok(Event::Eof) => break,
            _ => {}
        }

        if !slot_replaced {
            // Write the original event to the output buffer
            // writer.write_event(event)?;
        }
    }

    // Write the final content to a new file
    fs::write("output.xhtml", &output_buffer)?;

    Ok(())
}
