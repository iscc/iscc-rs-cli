extern crate reqwest;

use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::error::Error;

struct Config {
    tikahost: String,
    tikaport: String,
}


fn tika_file_data(file: &str) -> std::io::Result<Vec<u8>> {
    let mut data = Vec::new();
    let mut f = File::open(format!("{}",file))?;
    f.read_to_end(&mut data)?;

    Ok(data)
}

pub fn put_file(url: &str, file: &str) -> Result<String,Box<dyn Error>> {
    let data = tika_file_data(file).unwrap();
        // This will POST a body of `foo=bar&baz=quux`
    let client = reqwest::Client::new();
    let bodytext = client.put(&format!("{}",url))
    .body(data)
    .header("accept", &format!("text/plain"))
    .send()?
    .text()?;
    Ok(bodytext)
}

pub fn detect(file: &str) -> Result<String,Box<dyn Error>> {
    put_file("http://localhost:9998/detect/stream", file)
}

pub fn language(file: &str) -> Result<String,Box<dyn Error>> {
    put_file("http://localhost:9998/language/string", file)
}

pub fn text(file: &str) -> Result<String,Box<dyn Error>> {
    put_file("http://localhost:9998/tika", file)
}

