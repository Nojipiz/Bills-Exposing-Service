use actix_web::Error;
use ini::Ini;
use std::fs::File;
use std::io::Read;

pub struct Settings {
    pub address: String,
    pub port: String,
    pub path: String,
    pub extension: String,
}

pub fn get_settings() -> Settings {
    let content = Ini::load_from_file("./conf.ini").unwrap();
    let server_section = content.section(Some("Server")).unwrap();
    let address = server_section.get("address").unwrap().to_owned();
    let port = server_section.get("port").unwrap().to_owned();
    let directory_section = content.section(Some("Directory")).unwrap();
    let path = directory_section.get("path").unwrap().to_owned();
    let extension = directory_section.get("extension").unwrap().to_owned();
    Settings {
        address,
        port,
        path,
        extension,
    }
}

pub fn get_bill_by_number(
    _path: String,
    _number: u32,
    _period: u32,
    _extension: String,
) -> Result<Vec<u8>, Error> {
    let full_path: String = _path + "FETU" + &_number.to_string() + &_extension; // TODO: Period Change
    print!("{}", &full_path);
    let mut file = File::open(full_path)?;
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content).unwrap();
    Ok(file_content)
}
