use log::{debug, error, info, warn};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use rusqlite::types::ValueRef;
use rusqlite::{Connection, Result, Row};
use serde::Serialize;
use serde_json::{Map, Value};
use std::collections::HashMap;

use crate::utils;

fn get_table_schema(conn: &Connection, table_name: &str) -> Result<Vec<(String, String)>> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table_name))?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(1)?, row.get::<_, String>(2)?)) // (column_name, column_type)
    })?;

    let mut schema = Vec::new();
    for row in rows {
        schema.push(row?);
    }
    Ok(schema)
}

fn row_to_json(columns: Vec<String>, row: &Row) -> Result<Map<String, Value>> {
    let mut map = Map::new();
    for (i, column) in columns.iter().enumerate() {
        let value: Value = match row.get_ref(i)? {
            rusqlite::types::ValueRef::Null => Value::Null,
            rusqlite::types::ValueRef::Integer(v) => Value::Number(v.into()),
            rusqlite::types::ValueRef::Real(v) => {
                serde_json::Number::from_f64(v).map_or(Value::Null, Value::Number)
            }
            rusqlite::types::ValueRef::Text(v) => {
                Value::String(String::from_utf8_lossy(v).to_string())
            }
            rusqlite::types::ValueRef::Blob(v) => {
                Value::Array(v.iter().map(|&b| Value::Number(b.into())).collect())
            }
        };
        map.insert(column.clone(), value);
    }
    Ok(map)
}

fn row_to_hashmap(columns: Vec<String>, row: &Row) -> rusqlite::Result<HashMap<String, String>> {
    let mut map = HashMap::new();
    for (i, column) in columns.iter().enumerate() {
        let value: String = match row.get_ref(i)? {
            ValueRef::Null => "null".to_string(),
            ValueRef::Integer(v) => v.to_string(),
            ValueRef::Real(v) => v.to_string(),
            ValueRef::Text(v) => String::from_utf8_lossy(v).to_string(),
            ValueRef::Blob(v) => v
                .iter()
                .map(|&b| format!("{:02x}", b))
                .collect::<Vec<String>>()
                .join(""),
        };
        map.insert(column.to_string(), value);
    }
    Ok(map)
}

fn query_to_map(
    conn: &Connection,
    query: String,
    table_name: &str,
) -> Result<Vec<HashMap<String, String>>> {
    let schema = get_table_schema(conn, table_name)?;
    let columns: Vec<String> = schema.iter().map(|(name, _)| name.clone()).collect();

    let mut stmt = conn.prepare(&query)?;
    let rows = stmt.query_map([], |row| row_to_hashmap(columns.clone(), row))?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }
    Ok(result)
}

fn process(
    sqlite_src: String,
    query: String,
    table_name: String,
) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    let conn = Connection::open(sqlite_src)?;
    let map_data = query_to_map(&conn, query, table_name.as_str())?;
    // let json_str = serde_json::to_string_pretty(&json_data)?;
    Ok(map_data)
}

pub fn gen_map(
    eingabe: Vec<u8>,
    vorlagen_dir: std::path::PathBuf,
    language: Option<&String>,
) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    let mut ausgabepuffer: Vec<u8> = Vec::new();
    let mut xmlschreiber = Writer::new(&mut ausgabepuffer);

    let eingabe_str = std::str::from_utf8(&eingabe)?;
    let mut xmlleser = Reader::from_str(eingabe_str);

    let mut fahne_sqlanfang = false;
    let mut fahne_sqlende = false;
    let mut fahne_backend = false;

    let mut sqlite_content = String::new();
    let mut sqlite_src = String::new();
    let mut sqlite_table = String::new();

    loop {
        match xmlleser.read_event() {
            Ok(Event::Start(e)) if e.local_name().as_ref() == b"sqlite" => {
                fahne_sqlanfang = true;
                if utils::attribut_vorhanden(e.clone(), "backend") {
                    fahne_backend = true;
                    if let Some(src) = utils::attributenwert_lesen(e.clone(), "src") {
                        sqlite_src = src;
                        if let Some(table) = utils::attributenwert_lesen(e.clone(), "table") {
                            sqlite_table = table;
                        }
                    }
                }
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

    if fahne_backend && fahne_sqlanfang && fahne_sqlende {
        info!("Content of <sqlite> tag: {}", sqlite_content);
        let map = process(sqlite_src, sqlite_content, sqlite_table).unwrap();
        Ok(map)
    } else {
        warn!("No <sqlite> tag found or it is not properly closed.");
        return Err("No <sqlite> tag found or it is not properly closed.".into());
    }
}

pub fn transform() {}
