use crate::persistence::Settings;
use reqwest::header::USER_AGENT;

extern crate public_ip;
extern crate reqwest;

const IPV4_CHARACTERS: usize = 16;
const ADDRESS_KEY: &str = "address";

pub async fn send_public_ip(settings: &Settings) -> Result<String, String> {
    let mut address = String::with_capacity(IPV4_CHARACTERS);
    if let Some(ip) = public_ip::addr().await {
        address.push_str(&ip.to_string());
        println!("Public ip address: {:?}", &address);
    } else {
        return Err("Couldn't get an IP address".to_owned());
    }
    let client = reqwest::Client::new();
    match client
        .post(&settings.middle_url)
        .header(USER_AGENT, &settings.agent_key)
        .header(ADDRESS_KEY, &address)
        .send()
        .await
    {
        Ok(it) => return Ok(it.status().to_string()),
        Err(err) => return Err(err.to_string()),
    };
}
