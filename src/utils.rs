pub use path::endung_mit_sprache_erweitern;
pub use xml::attribut_vorhanden;
pub use xml::attributenwert_lesen;

pub mod xml {
    use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
    use quick_xml::Reader;
    use std::borrow::Cow;
    use std::io::Cursor;

    pub fn find_xml_tags<'a>(
        xml: String,
        tag_name: &'a str,
    ) -> impl Iterator<Item = BytesStart<'a>> {
        let cursor = Cursor::new(xml);
        let mut reader = Reader::from_reader(cursor);
        let mut buf = Vec::new();

        std::iter::from_fn(move || loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.local_name().as_ref() == tag_name.as_bytes() => {
                    return Some(e.to_owned());
                }
                Ok(Event::Eof) => return None,
                Err(_) => return None,
                _ => (),
            }
            buf.clear();
        })
    }

    pub fn attribut_vorhanden(element: BytesStart, attributename: &str) -> bool {
        if let Some(_) = element.attributes().find(|attr| {
            attr.clone().unwrap().key.local_name().as_ref() == attributename.as_bytes()
        }) {
            true
        } else {
            false
        }
    }

    pub fn attributenwert_lesen(element: BytesStart, attributename: &str) -> Option<String> {
        if let Some(attribute) = element.attributes().find(|attr| {
            attr.clone().unwrap().key.local_name().as_ref() == attributename.as_bytes()
        }) {
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
}
pub mod path {
    use std::path::PathBuf;

    pub fn endung_mit_sprache_erweitern(path: &PathBuf, language: Option<&String>) -> PathBuf {
        let mut new_path = path.clone();
        if let Some(lang) = language {
            if let Some(extension) = path.extension() {
                let mut new_extension = std::ffi::OsString::new();
                new_extension.push(lang);
                new_extension.push(".");
                new_extension.push(extension);
                new_path.set_extension(new_extension);
            }
        }
        new_path
    }
}
