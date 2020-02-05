extern crate reqwest;
extern crate serde_json;
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

//create and return tika configuration
pub fn config(host: &str, port: &str, active: bool) -> Result<TikaConfig, Box<dyn Error>> {
    let ret = TikaConfig {
        host: host.to_string(),
        port: port.to_string(),
        active,
    };
    Ok(ret)
}

//check if tika is available
pub fn check(config: &TikaConfig) -> Result<String, Box<dyn Error>> {
    let url = format!("http://{}:{}/tika", config.host, config.port);
    get(&url)
}

//"get" url
pub fn get(url: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let bodytext = client.get(url).send()?.text()?;
    Ok(bodytext)
}

//read the file data
fn tika_file_data(file: &str) -> std::io::Result<Vec<u8>> {
    let mut data = Vec::new();
    let mut f = File::open(file.to_string())?;
    f.read_to_end(&mut data)?;
    Ok(data)
}

//"put" file to url
pub fn put_file(url: &str, file: &str) -> Result<String, Box<dyn Error>> {
    let data = tika_file_data(file).unwrap();
    // This will POST a body of `foo=bar&baz=quux`
    let client = reqwest::blocking::Client::new();
    let bodytext = client
        .put(url)
        .body(data)
        .header("accept", "text/plain".to_string())
        .send()?
        .text()?;
    Ok(bodytext)
}

//detect the mediatype of a file
pub fn detect(config: &TikaConfig, file: &str) -> Result<String, Box<dyn Error>> {
    put_file(
        &format!("http://{}:{}/detect/stream", config.host, config.port),
        file,
    )
}

//detect the language of a file
pub fn language(config: &TikaConfig, file: &str) -> Result<String, Box<dyn Error>> {
    put_file(
        &format!("http://{}:{}/language/string", config.host, config.port),
        file,
    )
}

//extract the text of a file
pub fn text(config: &TikaConfig, file: &str) -> Result<String, Box<dyn Error>> {
    put_file(
        &format!("http://{}:{}/tika", config.host, config.port),
        file,
    )
}

//get the metadata json object
pub fn metadata(config: &TikaConfig, file: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let data = tika_file_data(file).unwrap();
    let client = reqwest::blocking::Client::new();
    let url = format!("http://{}:{}/meta", config.host, config.port);
    let metajson: serde_json::Value = client
        .put(&url)
        .body(data)
        .header("accept", "application/json".to_string())
        .send()?
        .json()?;
    Ok(metajson)
}

pub fn title(config: &TikaConfig, file: &str) -> Result<String, Box<dyn Error>> {
    let meta = metadata(&config, &file)?;
    let mut title: String = "".to_string();
    let search = "title";
    let mut titlecandidates = vec![];

    let current_path = vec![];
    searchkey(&meta, current_path, &mut titlecandidates, &search, false);
    if !titlecandidates.is_empty() {
        title = titlecandidates[0].to_string();
    }
    Ok(title)
}

fn searchkey(
    value: &serde_json::Value,
    current_path: Vec<String>,
    output: &mut Vec<String>,
    search: &str,
    hunt: bool,
) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let mut new_path = current_path.clone();
                new_path.push(k.to_owned());
                if k.to_string().find(search) >= Some(0) {
                    if !v.to_string().trim().is_empty() {
                        output.push(v.to_string().trim().to_string());
                    } else {
                        searchkey(v, new_path, output, search, true);
                    }
                } else if v.is_string() && hunt {
                    if !v.to_string().trim().is_empty() {
                        output.push(v.to_string().trim().to_string());
                    }
                } else {
                    searchkey(v, new_path, output, search, hunt);
                }
            }
        }
        serde_json::Value::Array(array) => {
            for (i, v) in array.iter().enumerate() {
                let mut new_path = current_path.clone();
                new_path.push(i.to_string().to_owned());
                if v.is_string() && hunt {
                    if !v.to_string().trim().is_empty() {
                        output.push(v.to_string().trim().to_string());
                    }
                } else {
                    searchkey(v, new_path, output, search, hunt);
                }
            }
        }
        _ => (),
    }
}
