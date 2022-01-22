use futures::Future;
use ini::Ini;

pub struct Settings {
    pub address: String,
    pub port: String,
    pub path: String,
}

fn read_config() -> Settings {
    let content = Ini::load_from_file("./conf.ini").unwrap();
    let server_section = content.section(Some("Server")).unwrap();
    let address = server_section.get("address").unwrap().to_owned();
    let port = server_section.get("port").unwrap().to_owned();
    let directory_section = content.section(Some("Directory")).unwrap();
    let path = directory_section.get("path").unwrap().to_owned();
    Settings {
        address,
        port,
        path,
    }
}
