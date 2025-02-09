use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::borrow::Cow;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use std::collections::HashMap;

pub fn werte_ersetzen(eingabe: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Schreiber definiert
    let mut output_buffer: Vec<u8> = Vec::new();
    let mut xml_writer = Writer::new(&mut output_buffer);

    // Leser definiert
    let mut xml_reader = Reader::from_str(&eingabe);

    let mut probe = HashMap::new();
    probe.insert("title", "discorsi");
    probe.insert("author", "Machiavelli");

    let mut column_anfang_fahne = false;

    loop {
        match xml_reader.read_event() {
            Ok(Event::Start(ereignis)) if ereignis.local_name().as_ref() == b"column" => {
                column_anfang_fahne = true;
                let mut wert: String;
                let mut tag: String;
                if let Some(name_attr) = ereignis
                    .attributes()
                    .find(|attr| attr.clone().unwrap().key.local_name().as_ref() == b"content")
                {
                    let wert_cow = name_attr.unwrap().value;
                    wert = match wert_cow {
                        Cow::Borrowed(borrowed) => String::from_utf8(borrowed.to_vec()).unwrap(),
                        Cow::Owned(owned) => String::from_utf8(owned).unwrap(),
                    };
                    if let Some(name_attr) = ereignis
                        .attributes()
                        .find(|attr| attr.clone().unwrap().key.local_name().as_ref() == b"tag")
                    {
                        let tag_cow = name_attr.unwrap().value;
                        tag = match tag_cow {
                            Cow::Borrowed(borrowed) => {
                                String::from_utf8(borrowed.to_vec()).unwrap()
                            }
                            Cow::Owned(owned) => String::from_utf8(owned).unwrap(),
                        };
                        let mut elem_start = BytesStart::new(tag.clone());
                        elem_start.extend_attributes(
                            ereignis
                                .attributes()
                                .filter(|attr| {
                                    attr.clone().unwrap().key.local_name().as_ref() == b"content"
                                })
                                .map(|attr| attr.unwrap()),
                        );

                        xml_writer.write_event(Event::Start(elem_start));

                        let w = match probe.get(wert.as_str()) {
                            Some(w) => w,
                            None => {
                                println!("Wert nicht vorhanden!");
                                &""
                            }
                        };

                        xml_writer.write_event(Event::Text(BytesText::new(w)));

                        let mut elem_end = BytesEnd::new(tag);
                        xml_writer.write_event(Event::End(elem_end));
                    }
                }
            }
            Ok(Event::End(ereignis))
                if ereignis.local_name().as_ref() == b"column" && column_anfang_fahne =>
            {
                column_anfang_fahne = false;
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
