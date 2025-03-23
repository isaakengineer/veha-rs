use comrak::{markdown_to_html, Options};
use markdown;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use crate::csvpilot;
use crate::utils;

pub fn werte_ersetzen(
    eingabe: String,
    vorlagen_dir: std::path::PathBuf,
    language: Option<&String>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Schreiber definiert
    let mut output_buffer: Vec<u8> = Vec::new();
    let mut xml_writer = Writer::new(&mut output_buffer);

    // Leser definiert
    let mut xml_reader = Reader::from_str(&eingabe);

    let mut md_anfang_fahne = false;

    let mut md_src_pfad;

    loop {
        match xml_reader.read_event() {
            Ok(Event::Start(e)) if e.local_name().as_ref() == b"md" => {
                md_anfang_fahne = true;
                let mut wert: String;
                let mut tag: String;

                if let Some(md_src) = csvpilot::attributenwert_lesen(e.clone(), "src") {
                    md_src_pfad = vorlagen_dir.join(md_src);
                    if utils::attribut_vorhanden(e.clone(), "multilingual") {
                        md_src_pfad = utils::endung_mit_sprache_erweitern(&md_src_pfad, language);
                    }
                    let mut file = std::fs::read_to_string(&md_src_pfad)
                        .expect("The path provieded via CLI could not be read!");
                    // let mut options = Options::default();
                    // options.extension.footnotes = true;
                    // let mut html = markdown_to_html(&file, &options);
                    let mut html = markdown::to_html_with_options(&file, &markdown::Options::gfm())
                        .unwrap_or("".to_string());

                    if let Some(tag) = csvpilot::attributenwert_lesen(e.clone(), "tag") {
                        let mut html_xml_reader = Reader::from_str(&html);

                        let mut elem_start = BytesStart::new(tag.clone());
                        elem_start.extend_attributes(
                            e.attributes()
                                .filter(|attr| {
                                    attr.clone().unwrap().key.local_name().as_ref() != b"src"
                                        && attr.clone().unwrap().key.local_name().as_ref() != b"tag"
                                })
                                .map(|attr| attr.unwrap()),
                        );

                        xml_writer.write_event(Event::Start(elem_start));
                        loop {
                            match html_xml_reader.read_event() {
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
            Ok(Event::End(e)) if e.local_name().as_ref() == b"md" && md_anfang_fahne => {
                md_anfang_fahne = false;
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
