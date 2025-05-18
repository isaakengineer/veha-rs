use csv;
use log::{debug, error, info, warn};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::borrow::Cow;
use std;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use rusqlite::types::ValueRef;
use rusqlite::{Connection, Result, Row};

use crate::utils;


pub fn abfragedurchfuehren(
	sqlite_src: String,
	abfrage: String,
	spalten: Vec<&str>,
) -> Result<Vec<HashMap<String, String>>> {
	let conn = Connection::open(sqlite_src)?;
	let mut stmt = conn.prepare(&abfrage)?;
	let rows = stmt.query_map([], |row| row_to_hashmap(spalten.clone(), row))?;

	let mut result = Vec::new();
	for row in rows {
		result.push(row?);
	}
	Ok(result)
}

pub fn werte_ersetzen(
		eingabe: String,
		records: HashMap<String, String>,
	) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
		// Schreiber definiert
		let mut output_buffer: Vec<u8> = Vec::new();
		let mut xml_writer = Writer::new(&mut output_buffer);

		// Leser definiert
		let mut xml_reader = Reader::from_str(&eingabe);

		// let mut probe = HashMap::new();
		// probe.insert("title", "discorsi");
		// probe.insert("author", "Machiavelli");

		let mut column_anfang_fahne = false;

		let mut tags: Vec<String> = Vec::new();
		let mut column_zahl = 0;

		loop {
			match xml_reader.read_event() {
				Ok(Event::Start(e)) if e.local_name().as_ref() == b"column" => {
					// println!("Anfag gefunden");
					column_anfang_fahne = true;
					column_zahl = column_zahl + 1;

					let mut wert: String;
					let mut tag: String;
					if let Some(wert) = utils::attributenwert_lesen(e.clone(), "content") {
						if let Some(tag) = utils::attributenwert_lesen(e.clone(), "tag") {
							if let Some(attribute) = utils::attributenwert_lesen(e.clone(), "attribute") {
								let mut elem_start = BytesStart::new(tag.clone());
								elem_start.extend_attributes(
									e.attributes()
										.filter(|attr| {
											attr.clone().unwrap().key.local_name().as_ref()
												!= b"content"
												&& attr.clone().unwrap().key.local_name().as_ref()
													!= b"tag"
												&& attr.clone().unwrap().key.local_name().as_ref()
													!= b"attribute"
										})
										.map(|attr| attr.unwrap()),
								);

								let w = match records.get(wert.as_str()) {
									Some(w) => w,
									None => {
										println!("Wert nicht vorhanden!");
										""
									}
								};
								elem_start.push_attribute((attribute.as_bytes(), w.as_bytes()));
								xml_writer.write_event(Event::Start(elem_start));
								tags.push(tag);
								// let mut elem_end = BytesEnd::new(tag);
								// xml_writer.write_event(Event::End(elem_end));
							} else {
								let mut elem_start = BytesStart::new(tag.clone());
								elem_start.extend_attributes(
									e.attributes()
										.filter(|attr| {
											attr.clone().unwrap().key.local_name().as_ref()
												!= b"content"
												&& attr.clone().unwrap().key.local_name().as_ref()
													!= b"tag"
										})
										.map(|attr| attr.unwrap()),
								);

								let w = match records.get(wert.as_str()) {
									Some(w) => w,
									None => {
										println!("Wert nicht vorhanden!");
										""
									}
								};
								xml_writer.write_event(Event::Start(elem_start));
								xml_writer.write_event(Event::Text(BytesText::new(w)));
								tags.push(tag);
								// let mut elem_end = BytesEnd::new(tag);
								// xml_writer.write_event(Event::End(elem_end));
							}
						}
					}
				}
				Ok(Event::End(e)) if e.local_name().as_ref() == b"column" => {
					column_anfang_fahne = false;
					column_zahl = column_zahl - 1;
					// println!("tags = {:?}", tags);
					let tag = tags.pop().unwrap();
					let mut elem_end = BytesEnd::new(tag);
					xml_writer.write_event(Event::End(elem_end));
				}
				Ok(Event::Eof) => break,
				Ok(e) => {
					xml_writer.write_event(e)?;
				}
				Err(e) => return Err(e.into()),
				_ => {}
			}
		}
		Ok(output_buffer)
}

fn row_to_hashmap(columns: Vec<&str>, row: &Row) -> rusqlite::Result<HashMap<String, String>> {
	let mut map = HashMap::new();
	for (i, column) in columns.iter().enumerate() {
		let value: String = match row.get_ref(i)? {
			ValueRef::Null => "null".to_string(),
			ValueRef::Integer(v) => v.to_string(),
			ValueRef::Real(v) => v.to_string(),
			ValueRef::Text(v) => String::from_utf8_lossy(v).to_string(),
			ValueRef::Blob(v) => v
				.iter()
				.map(|&b| format!("{:02x}", b))
				.collect::<Vec<String>>()
				.join(""),
		};
		map.insert(column.to_string(), value);
	}
	Ok(map)
}
