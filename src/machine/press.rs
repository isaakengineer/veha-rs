use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader;
use std::collections::HashMap;
use std::error::Error;
use std::str::Bytes;

use crate::utils;

pub fn hashmap_drucken(
    eingabe: Vec<u8>,
    map: HashMap<String, String>,
    tagname: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut xmlausgabepuffer: Vec<u8> = Vec::new();
    let mut xmlschreiber = quick_xml::Writer::new(&mut xmlausgabepuffer);

    let eingabe_str = String::from_utf8(eingabe)?;
    let mut xmlleser = Reader::from_str(&eingabe_str);

    let mut column_anfang_fahne = false;
    let mut tags: Vec<String> = Vec::new();
    let mut column_zahl = 0;

    loop {
        match xmlleser.read_event() {
            Ok(Event::Start(e)) if e.local_name().as_ref() == tagname => {
                // println!("Anfag gefunden");
                column_anfang_fahne = true;
                column_zahl = column_zahl + 1;

                let mut wert: String;
                let mut tag: String;
                if let Some(wert) = utils::attributenwert_lesen(e.clone(), "content") {
                    if let Some(tag) = utils::attributenwert_lesen(e.clone(), "tag") {
                        if let Some(attribute) = utils::attributenwert_lesen(e.clone(), "attribute")
                        {
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

                            let w = match map.get(wert.as_str()) {
                                Some(w) => w,
                                None => {
                                    println!("Wert nicht vorhanden!");
                                    ""
                                }
                            };
                            elem_start.push_attribute((attribute.as_bytes(), w.as_bytes()));
                            xmlschreiber.write_event(Event::Start(elem_start));
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

                            let w = match map.get(wert.as_str()) {
                                Some(w) => w,
                                None => {
                                    println!("Wert nicht vorhanden!");
                                    ""
                                }
                            };
                            xmlschreiber.write_event(Event::Start(elem_start));
                            xmlschreiber.write_event(Event::Text(BytesText::new(w)));
                            tags.push(tag);
                            // let mut elem_end = BytesEnd::new(tag);
                            // xml_writer.write_event(Event::End(elem_end));
                        }
                    }
                }
            }
            Ok(Event::End(e)) if e.local_name().as_ref() == tagname => {
                column_anfang_fahne = false;
                column_zahl = column_zahl - 1;
                // println!("tags = {:?}", tags);
                let tag = tags.pop().unwrap();
                let mut elem_end = BytesEnd::new(tag);
                xmlschreiber.write_event(Event::End(elem_end));
            }
            Ok(Event::Eof) => break,
            Ok(e) => {
                xmlschreiber.write_event(e)?;
            }
            Err(e) => return Err(e.into()),
            _ => {}
        }
    }
    Ok(xmlausgabepuffer)
}

pub fn verteilen_nach_tag(
    xmleingabe: &str,
    tagname: &str,
) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let mut reader = Reader::from_str(xmleingabe);

    let mut backend_content = String::new();
    let mut outside_content = String::new();
    let mut in_backend = false;
    let mut backend_count = 0;

    let mut xmlausserhalbpuffer: Vec<u8> = Vec::new();
    let mut xmlassuerhalbschreiber = quick_xml::Writer::new(&mut xmlausserhalbpuffer);

    let mut xmlinnerhalbpuffer: Vec<u8> = Vec::new();
    let mut xmlinnerhalbschreiber = quick_xml::Writer::new(&mut xmlinnerhalbpuffer);

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == tagname.as_bytes() => {
                if backend_count > 0 {
                    return Err(format!("More than one <{:?}> tag found", tagname).into());
                }
                in_backend = true;
                backend_count += 1;
            }
            Ok(Event::End(ref e)) if e.local_name().as_ref() == tagname.as_bytes() => {
                in_backend = false;
            }
            Ok(Event::Eof) => break,
            Ok(event) => {
                if in_backend {
                    xmlinnerhalbschreiber.write_event(event)?;
                } else {
                    xmlassuerhalbschreiber.write_event(event)?;
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok((xmlinnerhalbpuffer, xmlausserhalbpuffer))
}
