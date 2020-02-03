extern crate reqwest;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str;

#[derive(Debug)]
pub struct TikaConfig {
    pub host: String,
    pub port: String,
    pub active: bool,
}

pub fn config(host: &str, port: &str, active: bool) -> Result<TikaConfig, Box<dyn Error>> {
    let ret = TikaConfig {
        host: host.to_string(),
        port: port.to_string(),
        active,
    };
    Ok(ret)
}

pub fn check(config: &TikaConfig) -> Result<String, Box<dyn Error>> {
    let url = format!("http://{}:{}/tika", config.host, config.port);
    get(&url)
}

pub fn get(url: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let bodytext = client.get(url).send()?.text()?;
    Ok(bodytext)
}

fn tika_file_data(file: &str) -> std::io::Result<Vec<u8>> {
    let mut data = Vec::new();
    let mut f = File::open(file.to_string())?;
    f.read_to_end(&mut data)?;
    Ok(data)
}

pub fn put_file(url: &str, file: &str) -> Result<String, Box<dyn Error>> {
    let data = tika_file_data(file).unwrap();
    // This will POST a body of `foo=bar&baz=quux`
    let client = reqwest::Client::new();
    let bodytext = client
        .put(url)
        .body(data)
        .header("accept", "text/plain".to_string())
        .send()?
        .text()?;
    Ok(bodytext)
}

pub fn detect(config: &TikaConfig, file: &str) -> Result<String, Box<dyn Error>> {
    put_file(
        &format!("http://{}:{}/detect/stream", config.host, config.port),
        file,
    )
}

pub fn language(config: &TikaConfig, file: &str) -> Result<String, Box<dyn Error>> {
    put_file(
        &format!("http://{}:{}/language/string", config.host, config.port),
        file,
    )
}

pub fn text(config: &TikaConfig, file: &str) -> Result<String, Box<dyn Error>> {
    put_file(
        &format!("http://{}:{}/tika", config.host, config.port),
        file,
    )
}
