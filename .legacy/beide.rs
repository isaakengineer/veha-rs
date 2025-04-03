use quick_xml::{de::from_str, se::to_string};
use std::fs;
use std::io::Result;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(unused)]
struct XmlData {
    records: Vec<Record>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(unused)]
struct Record {
    id: String,
    value: String,
}

pub fn probe() -> Result<()> {

	println!("reading the file");
	// Read the XML file
    let xml_data = fs::read_to_string("input.xml")?;
    println!("Data is: {:}", xml_data.clone());
    let mut data: XmlData = from_str(&xml_data).unwrap();
    println!("{:?}", data);
    // Modify the XML content
    for record in &mut data.records {
        if record.id == "1" {
            record.value = "UpdatedValue".to_string();
        }
    }

    // Write to a new XML file
    let modified_xml = to_string(&data).unwrap();
    println!("modified XML: {}",modified_xml);
    fs::write("output.xml", modified_xml)?;
	Ok(())
}
