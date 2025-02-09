use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn transform() -> Result<(), Box<dyn std::error::Error>> {
	let buch_xml_path = Path::new("./beispiel/buch/eintrag.xhtml");
	let mut vorlagen_dir = Path::new("./beispiel/vorlagen");

	// Read the content of buch/eintrag.xhtml
	let mut file = fs::read_to_string(buch_xml_path)?;
	// let mut reader = BufReader::new(file);
	let mut xml_reader = Reader::from_str(&file);
	xml_reader.config_mut().trim_text(true);

	// Create a vector to hold the processed XML data
	let mut output_buffer: Vec<u8> = Vec::new();
	let mut writer = Writer::new(&mut output_buffer);

	let mut slot_replaced = false;

	// for event in xml_reader.read_event() {
		// match event? {
	loop {
   		match xml_reader.read_event() {
			Ok(Event::Start(e)) if e.name.local_name == b"vorlage" => {
				// Extract the value of the 'name' attribute
				if let Some(name_attr) = e.attributes().find(|attr| attr.unwrap().key.as_ref() == b"name") {
					let template_name = name_attr.unwrap().value;

					// Construct the path to the template file
					let template_path = vorlagen_dir.join(template_name.as_ref()).with_extension("xhtml");
					if template_path.exists() {
						// Read and process the template file
						let template_content = fs::read_to_string(&template_path)?;

						// Replace <slot></slot> with content from the original file

						// let mut template_reader = BufReader::new(template_content.as_bytes());
						let mut template_xml_reader = Reader::from_str(&template_content);
						template_xml_reader.config_mut().trim_text(true);
						// for event in events {
							// match event? {
						loop {
						   	match template_xml_reader.read_event() {
								Ok(Event::Start(start)) if start.name().as_ref() == b"slot" => {
									// Write the <slot> tag
									writer.write_event(Event::Start(start.clone()))?;

									// Read and write everything inside <slot>
									let mut end_found = false;
									let template_xml_reader_c = template_xml_reader.clone();
									loop {
										match template_xml_reader.read_event() {
											Ok(event) => {
												// if event.name() == start.name() {
												// 	end_found = true;
												// 	break;
												// }
												writer.write_event(event)?;
											}
											Err(e) => return Err(e.into()),
											Ok(Event::Eof) => return Err("Premature EOF".into()),
										}
									}

									// Write the </slot> tag
									if end_found {
										writer.write_event(Event::End(BytesEnd::new("slot")))?;
									} else {
										return Err("No corresponding </slot> found".into());
									}

									slot_replaced = true;

								}
								Ok(Event::End(e)) if !slot_replaced => {
									// Write everything else from the template file as is
									writer.write_event(Event::End(e))?;
								}
								Ok(event) => {
									// write everything else from template file as is
									writer.write_event(event)?;
								}
								Err(e) => return Err(e.into()),
								Ok(Event::Eof) => break,
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
