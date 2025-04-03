use csv;
use log::{debug, error, info, warn};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use crate::utils;

pub fn attributenwert_lesen(element: BytesStart, attributename: &str) -> Option<String> {
    if let Some(attribute) = element
        .attributes()
        .find(|attr| attr.clone().unwrap().key.local_name().as_ref() == attributename.as_bytes())
    {
        let src_name = attribute.unwrap().value;
        let wert: String = match src_name {
            Cow::Borrowed(borrowed) => String::from_utf8(borrowed.to_vec()).unwrap(),
            Cow::Owned(owned) => String::from_utf8(owned).unwrap(),
        };
        Some(wert)
    } else {
        None
    }
}

pub fn csv_tag_einfuellen(
    eingabe: String,
    vorlagen_dir: std::path::PathBuf,
    language: Option<&String>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Schreiber definiert
    let mut output_buffer: Vec<u8> = Vec::new();
    let mut xml_writer = Writer::new(&mut output_buffer);

    // Leser definiert
    let mut xml_reader = Reader::from_str(&eingabe);

    let mut csv_anfang_fahne = false;
    let mut csv_ende_fahne = false;

    let mut csv_pfad: std::path::PathBuf;

    let mut csv_datei_pfad: Option<std::path::PathBuf> = None;
    let mut csv_datei_pfad_src;
    let mut xml_row_pfad: std::path::PathBuf;

    let mut schrift: Vec<u8> = Vec::new();

    loop {
        match xml_reader.read_event() {
            Ok(Event::Start(csv)) if csv.local_name().as_ref() == b"csv" => {
                csv_anfang_fahne = true; // TODO: check if the flag is already true, there is a mistake here!
                if let Some(csv_name) = attributenwert_lesen(csv.clone(), "src") {
                    // csv_datei_pfad_src.join(csv_name).with_extension("csv");
                    csv_datei_pfad_src = vorlagen_dir.join(csv_name);
                    if utils::attribut_vorhanden(csv, "multilingual") {
                        csv_datei_pfad_src =
                            utils::endung_mit_sprache_erweitern(&csv_datei_pfad_src, language);
                    }
                    csv_datei_pfad = Some(csv_datei_pfad_src);
                    println!("1. csv file {:?}", &csv_datei_pfad);
                }
            }
            Ok(Event::Start(row)) if row.local_name().as_ref() == b"row" && csv_anfang_fahne => {
                if let Some(row_pfad) = attributenwert_lesen(row.clone(), "src") {
                    if let Some(tag) = attributenwert_lesen(row.clone(), "tag") {
                        let mut notiz;
                        xml_row_pfad = vorlagen_dir.join(row_pfad);
                        println!("2. csv file {:?}", &csv_datei_pfad);
                        if let Some(csv_pfad) = csv_datei_pfad.clone() {
                            notiz = reihe_einfuellen(xml_row_pfad, csv_pfad)
                                .expect("Reihen konnte nicht eingefüllt werden!");
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

                            xml_writer.write_event(Event::Start(elem_start));

                            let mut csv_xml_reader = Reader::from_str(
                                std::str::from_utf8(&notiz).expect("converstion failed"),
                            );

                            loop {
                                match csv_xml_reader.read_event() {
                                    Ok(Event::Eof) => break,
                                    Ok(e) => {
                                        xml_writer.write_event(e)?;
                                    }
                                    Err(e) => return Err(e.into()),
                                }
                            }

                            let mut elem_end = BytesEnd::new(tag);
                            xml_writer.write_event(Event::End(elem_end));
                        }
                    }
                }
            }
            Ok(Event::End(e)) if e.local_name().as_ref() == b"row" && csv_anfang_fahne => {
                // TODO: quick check
            }
            Ok(Event::End(ereignis))
                if ereignis.local_name().as_ref() == b"csv" && csv_anfang_fahne =>
            {
                csv_anfang_fahne = false;
                csv_ende_fahne = true;
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

pub fn reihe_einfuellen(
    xml_pfad: std::path::PathBuf,
    csv_pfad: std::path::PathBuf,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    println!("replacing line from {:?} to {:?}", &xml_pfad, &csv_pfad);

    let mut res: Vec<u8> = Vec::new();
    let file = std::fs::File::open(csv_pfad)?;
    let mut rdr = csv::Reader::from_reader(&file);
    type Record = HashMap<String, String>;
    let mut resultatteile;

    let mut file =
        std::fs::read_to_string(&xml_pfad).expect("The path provieded via CLI could not be read!");

    for result in rdr.deserialize() {
        let record: Record = result?;
        resultatteile =
            werte_ersetzen(file.clone(), record).expect("etwas mit Reihen ersetzen stimmt nicht!");
        res.extend(resultatteile);
    }
    Ok(res)
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
                if let Some(wert) = attributenwert_lesen(e.clone(), "content") {
                    if let Some(tag) = attributenwert_lesen(e.clone(), "tag") {
                        if let Some(attribute) = attributenwert_lesen(e.clone(), "attribute") {
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
