extern crate mime_guess;
extern crate clap;
extern crate walkdir;
use std::error::Error;
static BATCH_MAX_DIRLEVEL: usize = 1000;

use iscc::{content_id_image, content_id_text, data_id, instance_id, meta_id};

use clap::{Arg, App, SubCommand, AppSettings};

use walkdir::WalkDir;

use std::path::Path;

use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    
    let matches = App::new("iscc-cli")
                          .setting(AppSettings::SubcommandRequiredElseHelp)
                          .version("0.1")
                          .author("Thilo Hille<hillethilo@gmail.com>")
                          .about("ISCC cli for the iscc-rs library (https://github.com/iscc/iscc-rs)")
                          .subcommand(SubCommand::with_name("gen")
                                      .about("Generate ISCC Code for FILE.")
                                      .version("0.1")
                                      .author("Thilo Hille<hillethilo@gmail.com>")
                                      .arg(Arg::with_name("file")
                                          .short("f")
                                          .long("file")
                                          .help("File to create ISCC code for.")
                                          .value_name("FILE")
                                          .takes_value(true)
                                          .required(true)
                                       )
                                      .arg(Arg::with_name("guess")
                                          .short("g")
                                          .help("Guess title (first line of text).")
                                      )
                                      .arg(Arg::with_name("title")
                                          .short("t")
                                          .long("title")
                                          .help("Title for Meta-ID creation.")
                                          .value_name("TEXT")
                                          .takes_value(true)
                                       )
                                      .arg(Arg::with_name("extra")
                                          .short("e")
                                          .long("extra")
                                          .help("Extra text for Meta-ID creation.")
                                          .value_name("TEXT")
                                          .takes_value(true)
                                       )
                            )
                          .subcommand(SubCommand::with_name("batch")
                                      .about("Create ISCC Codes for all files in PATH.")
                                      .version("0.1")
                                      .author("Thilo Hille<hillethilo@gmail.com>")
                                      .arg(Arg::with_name("path")
                                          .short("p")
                                          .long("path")
                                          .help("Path to create iscc codes for.")
                                          .value_name("PATH")
                                          .takes_value(true)
                                          .required(true)
                                       )
                                      .arg(Arg::with_name("recursive")
                                          .short("r")
                                          .long("recursive")
                                          .help("Recurse into subdirectories.")
                                       )
                                      .arg(Arg::with_name("guess")
                                          .short("g")
                                          .long("guess")
                                          .help("Guess title (first line of text).")
                                       )
                            )
                            .arg(Arg::with_name("v")
                                   .short("v")
                                   .multiple(true)
                                   .help("Sets the level of verbosity")
                            )
                          .get_matches();
    //todo: figure out howto do different verbosity output 
    //let VERBOSITY_LEVEL = matches.occurrences_of("v");

    // command configuration and execution
    if let Some(matches) = matches.subcommand_matches("gen") {
        let cmd = Command::Gen(
            matches.value_of("file").unwrap_or("").to_string(), 
            matches.value_of("title").unwrap_or("").to_string(), 
            matches.value_of("extra").unwrap_or("").to_string(),
            matches.is_present("guess"),
            false,
        );
        if matches.is_present("file") {
            cmd.execute()?;
        }
        Ok(())
    }
    else if let Some(matches) = matches.subcommand_matches("batch") {
        let cmd = Command::Batch(
            matches.value_of("path").unwrap_or(".").to_string(), 
            matches.is_present("recursive"), 
            matches.is_present("guess"),
        );
        if matches.is_present("path") {
            cmd.execute()?;
        }
        Ok(())
    }
    else{
        Ok(())
    }
}

enum Command {
    //Gen (file, title, extra, guess, detail)
    Gen (String, String, String, bool, bool),
    //Batch (recurse, guess) 
    Batch (String, bool, bool),
}

/*
struct ContentId {
    code: String,
    gmt: String,
    file: Path,
}
*/

impl Command {
    fn execute(&self) -> Result<String,Box<dyn Error>> {
        match self {
            Command::Gen(file, title, extra, _guess, showdetail) => {
                //eprintln!("Generating {} {} {}",file, title, extra);
                let cid = get_content_id(file, false)?;
                let (mid, _title, _extra) = meta_id(title, extra);
                let did = data_id(file)?;
                let (iid, _tophash) = instance_id(file)?;

                // Join ISCC Components to fully qualified ISCC Code
                let iscc_code = [mid, cid, did, iid].join("-");
                if *showdetail {
                    let filename = Path::new(file).file_name().unwrap();
                    println!("ISCC: {},{:?},<Todo>", iscc_code, filename);
                }
                else{
                    println!("ISCC: {}", iscc_code);
                }
                Ok(iscc_code)
            },
            Command::Batch(dir, recurse, guess) => {
                //eprintln!("Batching {} {} {}",dir, recurse, guess);
                //let walklevel: usize = BATCH_MAX_ITER;
                let walklevel = match recurse {
                    true => BATCH_MAX_DIRLEVEL,
                    false => 1,
                };
                for e in WalkDir::new(dir).max_depth(walklevel).into_iter().filter_map(|e| e.ok()) {
                    if e.metadata().unwrap().is_file() {
                        //eprint!("{}".e.path().display().unwrap());
                        let cmd = Command::Gen(
                            e.path().display().to_string(), 
                            "".to_string(), 
                            "".to_string(),
                            *guess,
                            true
                        );
                        let res = cmd.execute();
                        match res {
                            Ok(_result)=>{()},
                            Err(error) =>{
                                    eprintln!("Error {:?}", error);
                                    ()
                                }
                        }
                    }
                }
                Ok("done".to_string())
            }
        }
    }
}

#[derive(Debug)]
enum GeneralMediaType {
    Text(String),
    Image(String),
    Audio(String),
    Video(String),
    Unknown,
}

fn get_gmt(file: &str) -> GeneralMediaType {
    let guess = mime_guess::from_path(file);
    //todo: fix unwrap, crashes on unknown extensions
    let mimetype = guess.first_raw().unwrap();
    eprintln!("mime-type: {}", mimetype);
    let mut parts = mimetype.split("/");
    let gmt = parts.next().unwrap();
    let ft = parts.next().unwrap();
    match gmt {
        "text"  => GeneralMediaType::Text(String::from(ft)),
        "image" => GeneralMediaType::Image(String::from(ft)),
        "audio" => GeneralMediaType::Audio(String::from(ft)),
        "video" => GeneralMediaType::Video(String::from(ft)),
        _       => GeneralMediaType::Unknown,
    }
}

fn get_content_id(file: &str, partial: bool) -> Result<String, String> {
    let mediatype = get_gmt(file);
    eprintln!("mediatype: {:?}", mediatype);
    match mediatype {
        GeneralMediaType::Text(_ft) => {  
            //no errorhandling in content_id_text 
            let contents = match fs::read_to_string(file) {
                Ok(content) => content,
                Err(error) => format!("Could not read file: {} {}",file, error).to_string(),
            };
            Ok(content_id_text(&contents, partial))
        },
        GeneralMediaType::Image(_ft) => {  
            match content_id_image(file, partial) {
                Ok(id) => Ok(id),
                image_error => Err(format!("Error creating content_id_image: {:?}",image_error).to_string())
            }
        },
        GeneralMediaType::Audio(_ft) => {  
            Err("Mediatype not implemented yet".to_string())
        },
        GeneralMediaType::Video(_ft) => {  
            Err("Mediatype not implemented yet".to_string())
        },
        _ => {
            Err("Unknown mediatype".to_string())
        }
    }
}
