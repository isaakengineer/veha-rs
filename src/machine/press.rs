use quick_xml::events::Event;
use quick_xml::Reader;
use std::error::Error;

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
                    xmlassuerhalbschreiber.write_event(event)?;
                } else {
                    xmlinnerhalbschreiber.write_event(event)?;
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok((xmlinnerhalbpuffer, xmlausserhalbpuffer))
}
