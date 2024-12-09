use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use exif::Reader;

pub fn extract(image_dir: &str) {
    let mut file_map: BTreeMap<String, String> = BTreeMap::new();

    match fs::read_dir(image_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        match File::open(&path) {
                            Ok(file) => {
                                let mut buf_reader = BufReader::new(file);
                                let exif_reader = Reader::new();
                                match exif_reader.read_from_container(&mut buf_reader) {
                                    Ok(metadata) => {
                                        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                                        println!("Processing EXIF data for file: {:?}", path.file_name().unwrap());
                                        for field in metadata.fields() {
                                            let raw_data = field.value.display_as(field.tag).to_string();

                                            if raw_data.starts_with("0x4153434949000000") {
                                                // Remove the header
                                                let hex_data = &raw_data[18..];
                                                // Convert HEX to ASCII
                                                if let Ok(ascii_data) = hex_to_ascii(hex_data) {
                                                    match decode_base64(&ascii_data) {
                                                        Ok(decoded_str) => {
                                                            println!("Decoded Base64: {}", decoded_str);

                                                            // Use regex to extract both parts between quotes
                                                            let re = regex::Regex::new(r#""([^"]+)""#).unwrap();
                                                            let mut parts = re.captures_iter(&decoded_str);

                                                            if let (Some(key_match), Some(value_match)) = (parts.next(), parts.next()) {
                                                                let key = key_match[1].to_string();
                                                                let value = value_match[1].to_string();

                                                                println!("Key: {}, Value: {}", key, value);

                                                                // Insert the key-value pair into the BTreeMap
                                                                println!("Inserting...");
                                                                file_map.insert(key, value);
                                                            } else {
                                                                eprintln!("Failed to extract key and value from: {}", decoded_str);
                                                            }
                                                        }
                                                        Err(e) => eprintln!("Base64 decoding failed: {}", e),
                                                    }
                                                } else {
                                                    eprintln!("Failed to convert HEX to ASCII.");
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => eprintln!("Failed to read EXIF data from {:?}: {}", path, e),
                                }
                            }
                            Err(e) => eprintln!("Failed to open file {:?}: {}", path, e),
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("Failed to read directory {}: {}", image_dir, e),
    }

    // Combine all values in sorted order to create the final message
    let mut message = String::new();
    for (_, v) in &file_map {
        message.push_str(format!(" Word slice: {}", v).as_str());
    }

    println!("Message: {}", message);
}

fn hex_to_ascii(hex: &str) -> Result<String, std::num::ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).map(|c| c as char))
        .collect()
}

fn decode_base64(encoded: &str) -> Result<String, String> {
    match base64::decode(encoded) {
        Ok(decoded_bytes) => match String::from_utf8(decoded_bytes) {
            Ok(decoded_str) => Ok(decoded_str),
            Err(e) => Err(format!("Failed to convert decoded bytes to UTF-8: {}", e)),
        },
        Err(e) => Err(format!("Base64 decoding error: {}", e)),
    }
}