use quick_xml::Writer;
use quick_xml::events::Event;
use std::io::Cursor;

fn main() {
    // Create a new writer
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    // Start the outer XML element
    writer.write_event(Event::Start(quick_xml::events::BytesStart::borrowed(b"root", 4))).unwrap();

    // Create a new tag and write a snippet into it
    writer.write_event(Event::Start(quick_xml::events::BytesStart::borrowed(b"new_tag", 7))).unwrap();
    
    // Write the XML snippet you want to append
    let snippet = r#"<child>Content</child>"#;
    writer.write_event(Event::Text(quick_xml::events::BytesText::from_snippet(snippet))).unwrap();
    
    // Close the new tag
    writer.write_event(Event::End(quick_xml::events::BytesEnd::borrowed(b"new_tag"))).unwrap();

    // Close the outer XML element
    writer.write_event(Event::End(quick_xml::events::BytesEnd::borrowed(b"root"))).unwrap();

    // Get the resulting XML
    let result = String::from_utf8(writer.into_inner().into_inner()).unwrap();
    println!("{}", result);
}