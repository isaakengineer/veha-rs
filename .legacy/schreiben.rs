use std::io::Cursor;
use quick_xml::{events::{BytesEnd, BytesStart, BytesText, Event}, Writer};

pub fn beispiel_person() ->Vec<u8> {
	// source: https://medium.com/@mikecode/rust-how-to-write-xml-in-rust-quick-xml-4f31c14fa023
	let mut writer = Writer::new(Cursor::new(Vec::new()));
	let mut elem1_start = BytesStart::new("person");
	elem1_start.push_attribute(("id", "1"));
	let	elem1_end = BytesEnd::new("person");
	let elem2_start = BytesStart::new("name");
	let elem2_text = BytesText::new("Jim");
	let	elem2_end = BytesEnd::new("name");
	writer.write_event(Event::Start(elem1_start)).unwrap();
	writer.write_event(Event::Start(elem2_start)).unwrap();
	writer.write_event(Event::Text(elem2_text)).unwrap();
	writer.write_event(Event::End(elem2_end)).unwrap();
	writer.write_event(Event::End(elem1_end)).unwrap();
	let result = writer.into_inner().into_inner();
	return result;
	// println!("{}", String::from_utf8(result).unwrap());
}
