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

use crate::utils;
use crate::motor::sqlite;
use crate::tabular;

pub const SQL_TAGNAME:&[u8] = b"sql";
pub const ERSTE_ATTRIBUT_NAME:&str = "src"; // URI zu SQL-Datei (SQL-Query)
pub const ZWEITE_ATTRIBUT_NAME:&str = "ref"; // URI zu SQLite-Datei (SQLite-DB)
pub const DRITTE_ATTRIBUT_NAME:&str = "table";

pub fn binden(
	eingabe: String,
	vorlagen_dir: std::path::PathBuf,
	language: Option<&String>, // Zukunft falls etwas für Eingabe nötig!
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	// Schreiber definiert
	let mut ausgabepuffer: Vec<u8> = Vec::new();
	let mut xmlschreiber = Writer::new(&mut ausgabepuffer);

	// Leser definiert
	let mut xmlleser = Reader::from_str(&eingabe);

	let mut tag_anfangfahne = false;
	let mut tag_endefahne = false;

	let mut datenbank_pfadwert: Option<std::path::PathBuf> = None;
	let mut datenbankpfad: std::path::PathBuf;
	let mut sqldatei_pfadwert: Option<std::path::PathBuf> = None;
	let mut sqldateipfad: std::path::PathBuf;
	let mut tabellenname: String = format!("");

	let mut xml_row_pfad: std::path::PathBuf;

	let mut reihen: Vec<HashMap<String, String>> = Vec::new();

	loop {

		match xmlleser.read_event() {
			Ok(Event::Start(b)) if b.local_name().as_ref() == SQL_TAGNAME => {
				tag_anfangfahne = true;
				// TODO: falls Anfangfahne ist doch `true` dann der Datei hat Fehler in sich; Ob wir es brichten sollten oder nicht beleibt öffnen!
				if let Some(sqldatei_uri) = utils::attributenwert_lesen(b.clone(), ERSTE_ATTRIBUT_NAME ) {
					sqldateipfad = vorlagen_dir.join(sqldatei_uri);
					if let Some(datenbank_uri) = utils::attributenwert_lesen(b.clone(), ZWEITE_ATTRIBUT_NAME ) {
						datenbankpfad = vorlagen_dir.join(datenbank_uri);
						if let Some(tablename) = utils::attributenwert_lesen(b.clone(), DRITTE_ATTRIBUT_NAME ) {
							info!("1. sql file {:?}", &sqldateipfad);
							info!("2. sqlite file {:?}", &datenbankpfad);
							info!("3. table name {:?}", &tablename);
							tabellenname = tablename;
							let mut sql_content = String::new();
							let mut sql_file = fs::File::open(&sqldateipfad).unwrap_or_else(|e| {
								error!("Failed to open the SQL file at path: {}. Error: {}",sqldateipfad.display(),e);
        						panic!("The path for SQL-file could not be opened!");
							});
							sql_file.read_to_string(&mut sql_content)
    .expect(&format!("The SQL file at location {} could not be read!", sqldateipfad.clone().display()));
							let gelesenereihen = sqlite::process(
								datenbankpfad.display().to_string(),
								sql_content,
								tabellenname,
							).unwrap();
							reihen.extend(gelesenereihen.iter().cloned());
						} else {
							warn!("The {:?} tag did not have a {}-attribute;\n It may not be rendered properly.", SQL_TAGNAME, DRITTE_ATTRIBUT_NAME);
						}
					} else {
						warn!("The {:?} tag did not have a {}-attribute;\n It may not be rendered properly.", SQL_TAGNAME, ZWEITE_ATTRIBUT_NAME);
					}
				} else {
					warn!("The {:?} tag did not have a {}-attribute;\n It may not be rendered properly.", SQL_TAGNAME, ERSTE_ATTRIBUT_NAME);
				}
			}
			Ok(Event::Start(row)) if row.local_name().as_ref() == b"row" && tag_anfangfahne => {
				if let Some(row_pfad) = utils::attributenwert_lesen(row.clone(), "src") {
					if let Some(tag) = utils::attributenwert_lesen(row.clone(), "tag") {
						// TODO: Dateisätze aus `reihen` lesen und wiederholt einfüllen > reihe_einfuellen()


						let mut xmlteilchen;
						xml_row_pfad = vorlagen_dir.join(row_pfad);

						xmlteilchen = reihe_einfuellen(reihen.clone(), xml_row_pfad).expect("Reihen konnte nicht eingefüllt werden!");
						let mut elem_start = BytesStart::new(tag.clone());
						elem_start.extend_attributes(
							row.attributes()
								.filter(|attr| {
									attr.clone().unwrap().key.local_name().as_ref() != b"src"
										&& attr.clone().unwrap().key.local_name().as_ref()
											!= b"tag"
								})
								.map(|attr| attr.unwrap()),
						);

						xmlschreiber.write_event(Event::Start(elem_start));

						let mut csv_xml_reader = Reader::from_str(
							std::str::from_utf8(&xmlteilchen).expect("converstion failed"),
						);

						loop {
							match csv_xml_reader.read_event() {
								Ok(Event::Eof) => break,
								Ok(e) => {
									xmlschreiber.write_event(e)?;
								}
								Err(e) => return Err(e.into()),
							}
						}

						let mut elem_end = BytesEnd::new(tag);
						xmlschreiber.write_event(Event::End(elem_end));
					}
				}
			}
			Ok(Event::End(e)) if e.local_name().as_ref() == b"row" && tag_anfangfahne => {
				// TODO: quick check
			}
			Ok(Event::End(e))
				if e.local_name().as_ref() == SQL_TAGNAME && tag_anfangfahne =>
			{
				tag_anfangfahne = false;
				tag_endefahne = true;
			}
			Ok(Event::Eof) => break,
			Ok(e) => {
				xmlschreiber.write_event(e)?;
			}
			Err(e) => return Err(e.into()),
			_ => {}
		}
	}
	Ok(ausgabepuffer)
}

pub fn reihe_einfuellen(
	reihen: Vec<HashMap<String, String>>,
	xmlpfad: std::path::PathBuf,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

	let mut ausgabe: Vec<u8> = Vec::new();
	let mut ausgabeteil;

	let mut xmldatei = std::fs::read_to_string(&xmlpfad).expect(&format!("The XML data path {:?} provieded via src attribute could not be read.", xmlpfad.clone().display()));

	for eintrag in reihen {
		ausgabeteil = tabular::werte_ersetzen(xmldatei.clone(), eintrag).expect("an error occurd while replacing values in XML data.");
		ausgabe.extend(ausgabeteil);
	}
	Ok(ausgabe)
}
