extern crate base64;
use actix_web::Error;
use ini::Ini;
use std::fs::File;
use std::io::Read;

pub struct Settings {
    pub address: String,
    pub port: String,
    pub path: String,
    pub extension: String,
    pub auth_value: String,
    pub middle_url: String,
}

pub fn get_settings() -> Settings {
    let content = Ini::load_from_file("./conf.ini").unwrap();
    let server_section = content.section(Some("Server")).unwrap();
    let address = server_section.get("address").unwrap().to_owned();
    let port = server_section.get("port").unwrap().to_owned();
    let middle_url = server_section.get("middle_url").unwrap().to_owned();
    let directory_section = content.section(Some("Directory")).unwrap();
    let path = directory_section.get("path").unwrap().to_owned();
    let extension = directory_section.get("extension").unwrap().to_owned();
    let security_section = content.section(Some("Security")).unwrap();
    let agent_key = security_section.get("auth").unwrap().to_owned();

    Settings {
        address,
        port,
        path,
        extension,
        auth_value: agent_key,
        middle_url,
    }
}

pub fn get_bill_in_base64(
    _path: String,
    _number: u32,
    _period: u32,
    _extension: String,
) -> Result<String, Error> {
    let full_path: String = _path + "FETU" + &_number.to_string() + &_extension; // TODO: Period Change
    let mut file = File::open(full_path)?;
    let mut binary_content = Vec::new();
    file.read_to_end(&mut binary_content).unwrap();
    let base64_content: String = base64::encode(binary_content);
    Ok(base64_content)
}
