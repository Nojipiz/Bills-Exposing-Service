use crate::persistence::Settings;

extern crate public_ip;
extern crate reqwest;

const IPV4_PORT_CHARACTERS: usize = 20;
const ADDRESS_KEY: &str = "address";
const AUTH_KEY: &str = "auth";

pub async fn send_public_ip(settings: &Settings) -> Result<String, String> {
    let mut address = String::with_capacity(IPV4_PORT_CHARACTERS);
    if let Some(ip) = public_ip::addr().await {
        address.push_str(&ip.to_string());
        address.push_str(":");
        address.push_str(&settings.port);
        println!("Public ip address: {:?}", &address);
    } else {
        return Err("Couldn't get an IP address".to_owned());
    }
    let client = reqwest::Client::new();
    match client
        .post(&settings.middle_url)
        .header(AUTH_KEY, &settings.auth_value)
        .header(ADDRESS_KEY, "127.0.0.1:8080") //CHANGE THIS LINE OR PROD
        .send()
        .await
    {
        Ok(it) => return Ok(it.status().to_string()),
        Err(err) => return Err(err.to_string()),
    };
}
