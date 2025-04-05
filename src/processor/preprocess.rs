const EXPANSION_SIGN: char = '+';
const COLLECTION_SIGN_START: char = '{';
const COLLECTION_SIGN_END: char = '}';

pub fn collection_name(pfad: std::path::PathBuf) -> Option<String> {
    let dateiname = pfad.file_stem().and_then(|name| name.to_str());
    log::info!("Dateiname = {:?}", dateiname.clone());
    if let Some(dateiname) = dateiname {
        if let Some(start) = dateiname.find(COLLECTION_SIGN_START) {
            if let Some(ende) = dateiname.find(COLLECTION_SIGN_END) {
                if start < ende && dateiname.chars().nth(start - 1) == Some(EXPANSION_SIGN) {
                    let key = &dateiname[start + 1..ende];
                    return Some(key.to_string());
                }
            }
        }
    }
    return None;

    // let endung = pfad.extension().and_then(|ext| ext.to_str());
    // match endung {
    //     Some("xhtml") | Some("json") => {
    //         // Check for the pattern in the file name
    //     }
    //     None => (false),
    // }
}

pub fn process() {}
