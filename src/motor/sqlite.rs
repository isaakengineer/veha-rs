use log::{debug, error, info, warn};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;

use crate::utils;

pub fn lesen(
    eingabe: String,
    vorlagen_dir: std::path::PathBuf,
    language: Option<&String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut ausgabepuffer: Vec<u8> = Vec::new();
    let mut xmlschreiber = Writer::new(&mut ausgabepuffer);

    let mut xmlleser = Reader::from_str(&eingabe);

    let mut fahne_sqlanfang = false;
    let mut fahne_sqlende = false;

    let mut sqlite_content = String::new();

    loop {
        match xmlleser.read_event() {
            Ok(Event::Start(e)) if e.local_name().as_ref() == b"sqlite" => {
                fahne_sqlanfang = true;
                if let Some(sqlite_src) = utils::attributenwert_lesen(e.clone(), "src") {}
            }
            Ok(Event::Text(e)) if fahne_sqlanfang && !fahne_sqlende => {
                sqlite_content.push_str(&e.unescape()?);
            }
            Ok(Event::End(e)) if e.local_name().as_ref() == b"sqlite" => {
                fahne_sqlende = true;
            }
            Ok(Event::Eof) => break,
            Ok(e) => {
                // xmlleser.write_event(e)?;
            }
            Err(e) => return Err(e.into()),
            _ => {}
        }
    }

    if fahne_sqlanfang && fahne_sqlende {
        info!("Content of <sqlite> tag: {}", sqlite_content);
    } else {
        warn!("No <sqlite> tag found or it is not properly closed.");
    }

    Ok(())
}

pub fn transform() {}
