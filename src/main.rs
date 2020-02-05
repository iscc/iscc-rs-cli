extern crate clap;
extern crate dotext;
extern crate html2text;
extern crate mime_guess;
extern crate walkdir;

pub mod tika;

use std::error::Error;
static BATCH_MAX_DIRLEVEL: usize = 1000;

use iscc::{content_id_image, content_id_text, data_id, instance_id, meta_id};

use clap::{App, AppSettings, Arg, SubCommand};

use walkdir::WalkDir;

use tika::request::TikaConfig;

use dotext::*;
use std::io::Read;

use std::path::Path;

use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("iscc-cli")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version("0.1")
        .author("Thilo Hille<hillethilo@gmail.com>")
        .about("ISCC cli for the iscc-rs library (https://github.com/iscc/iscc-rs)")
        .subcommand(
            SubCommand::with_name("gen")
                .about("Generate ISCC Code for FILE.")
                .version("0.1")
                .author("Thilo Hille<hillethilo@gmail.com>")
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .help("File to create ISCC code for.")
                        .value_name("FILE")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("guess")
                        .short("g")
                        .help("Guess title (first line of text)."),
                )
                .arg(
                    Arg::with_name("title")
                        .short("t")
                        .long("title")
                        .help("Title for Meta-ID creation.")
                        .value_name("TEXT")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("extra")
                        .short("e")
                        .long("extra")
                        .help("Extra text for Meta-ID creation.")
                        .value_name("TEXT")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("batch")
                .about("Create ISCC Codes for all files in PATH.")
                .version("0.1")
                .author("Thilo Hille<hillethilo@gmail.com>")
                .arg(
                    Arg::with_name("dir")
                        .short("d")
                        .long("dir")
                        .help("Dirctory to create iscc codes for.")
                        .value_name("PATH")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("recursive")
                        .short("r")
                        .long("recursive")
                        .help("Recurse into subdirectories."),
                )
                .arg(
                    Arg::with_name("guess")
                        .short("g")
                        .long("guess")
                        .help("Guess title (first line of text)."),
                ),
        )
        .arg(
            Arg::with_name("tika")
                .short("k")
                .long("tika")
                .help("Use Apache Tika for media-type detection and text-extraction"),
        )
        .arg(
            Arg::with_name("host")
                .short("h")
                .long("host")
                .help("Hostname or Ipaddress of a Apache Tika Server (default: localhost)")
                .value_name("TIKAHOST")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("Port of a Apache Tika Server (default: 9998)")
                .value_name("PORT")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();
    //todo: figure out howto do different verbosity output
    //let VERBOSITY_LEVEL = matches.occurrences_of("v");

    // command configuration and execution
    let tikaconfig = tika::request::config(
        matches.value_of("host").unwrap_or("localhost"),
        matches.value_of("port").unwrap_or("9998"),
        matches.is_present("tika"),
    )
    .unwrap();
    if matches.is_present("tika") {
        tika::request::check(&tikaconfig)?;
        eprintln!(
            "Found tikaserver at {}:{}",
            tikaconfig.host, tikaconfig.port
        );
    }
    if let Some(matches) = matches.subcommand_matches("gen") {
        let file = matches.value_of("file").unwrap_or("").to_string();
        let title = matches.value_of("title").unwrap_or("").to_string();
        let extra = matches.value_of("extra").unwrap_or("").to_string();
        let guess = matches.is_present("guess");
        let showdetail = false;
        let cmd = Command::Gen(&file, &title, &extra, &guess, &showdetail, &tikaconfig);
        if matches.is_present("file") {
            cmd.execute()?;
        }
        Ok(())
    } else if let Some(matches) = matches.subcommand_matches("batch") {
        let dir = matches.value_of("dir").unwrap_or("").to_string();
        let recursive = matches.is_present("recursive");
        let guess = matches.is_present("guess");
        let cmd = Command::Batch(&dir, &recursive, &guess, &tikaconfig);
        if matches.is_present("tika") {
            cmd.execute()?;
        }
        if matches.is_present("dir") {
            cmd.execute()?;
        }
        Ok(())
    } else {
        Ok(())
    }
}

enum Command<'a> {
    //Gen (file, title, extra, guess, detail)
    Gen(
        &'a String,
        &'a String,
        &'a String,
        &'a bool,
        &'a bool,
        &'a TikaConfig,
    ),
    //Batch (recurse, guess)
    Batch(&'a String, &'a bool, &'a bool, &'a TikaConfig),
}

impl Command<'_> {
    fn execute(&self) -> Result<String, Box<dyn Error>> {
        match self {
            Command::Gen(
                ref file,
                ref title,
                ref extra,
                ref guess,
                ref showdetail,
                ref tikaconfig,
            ) => {
                //eprintln!("Generating {} {} {}",file, title, extra);

                let iscc = get_iscc_id(&file, false, &title, &extra, **guess, tikaconfig)?;

                // Join ISCC Components to fully qualified ISCC Code
                let iscc_code = [iscc.mid, iscc.cid, iscc.did, iscc.iid].join("-");
                if **showdetail {
                    let mut filename = "";
                    if let Some(i) = Path::new(&file).file_name().unwrap().to_str() {
                        filename = i;
                    }
                    println!(
                        "ISCC:{},{},{},{},{}",
                        iscc_code, iscc.tophash, filename, iscc.gmt, iscc.title
                    );
                } else {
                    println!("ISCC:{}", iscc_code);
                }
                Ok(iscc_code)
            }
            Command::Batch(ref dir, ref recurse, ref guess, ref tikaconfig) => {
                //eprintln!("Batching {} {} {}",dir, recurse, guess);
                //let walklevel: usize = BATCH_MAX_ITER;
                let walklevel = match recurse {
                    true => BATCH_MAX_DIRLEVEL,
                    false => 1,
                };
                for e in WalkDir::new(dir)
                    .max_depth(walklevel)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    if e.metadata().unwrap().is_file() {
                        //eprint!("{}".e.path().display().unwrap());
                        let file = e.path().display().to_string();
                        let title = "".to_string();
                        let extra = "".to_string();
                        let detail = true;
                        let cmd = Command::Gen(&file, &title, &extra, &guess, &detail, &tikaconfig);
                        let res = cmd.execute();
                        match res {
                            Ok(_result) => (),
                            Err(error) => {
                                eprintln!("Error {:?}", error);
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
}
impl GeneralMediaType {
    fn is_tika_extract(&self) -> bool {
        match self {
            GeneralMediaType::Text(_ft) => true,
            _ => false,
        }
    }

    fn extract(&self, file: &str) -> Result<(String, String, String), Box<dyn Error>> {
        match self {
            GeneralMediaType::Text(_ft) if _ft == "plain" => {
                let contents = fs::read_to_string(file)?;
                let mut firstline = "";
                for l in contents.lines() {
                    if l.trim() != "" {
                        firstline = l;
                        break;
                    }
                }
                Ok((contents.to_string(), firstline.to_string(), "".to_string()))
            }
            GeneralMediaType::Text(_ft) if _ft == "html" => {
                let width: usize = 72;
                let htmlfile = std::fs::File::open(file)?;
                let contents = html2text::from_read(htmlfile, width);
                let mut firstline = "";
                for l in contents.lines() {
                    if l.trim() != "" {
                        firstline = l;
                        break;
                    }
                }
                Ok((contents.to_string(), firstline.to_string(), "".to_string()))
            }
            GeneralMediaType::Text(_ft)
                if _ft == "vnd.openxmlformats-officedocument.wordprocessingml.document" =>
            {
                let mut mediafile = Docx::open(file)?;
                let mut contents = String::new();
                let _ = mediafile.read_to_string(&mut contents);
                let mut firstline = "";
                for l in contents.lines() {
                    if l.trim() != "" {
                        firstline = l;
                        break;
                    }
                }
                Ok((contents.to_string(), firstline.to_string(), "".to_string()))
            }
            GeneralMediaType::Text(_ft)
                if _ft == "vnd.openxmlformats-officedocument.spreadsheetml.sheet" =>
            {
                let mut mediafile = Xlsx::open(file)?;
                let mut contents = String::new();
                let _ = mediafile.read_to_string(&mut contents);
                let mut firstline = "";
                for l in contents.lines() {
                    if l.trim() != "" {
                        firstline = l;
                        break;
                    }
                }
                Ok((contents.to_string(), firstline.to_string(), "".to_string()))
            }
            GeneralMediaType::Text(_ft)
                if _ft == "vnd.openxmlformats-officedocument.presentationml.presentation" =>
            {
                let mut mediafile = Pptx::open(file)?;
                let mut contents = String::new();
                let _ = mediafile.read_to_string(&mut contents);
                let mut firstline = "";
                for l in contents.lines() {
                    if l.trim() != "" {
                        firstline = l;
                        break;
                    }
                }
                Ok((contents.to_string(), firstline.to_string(), "".to_string()))
            }
            GeneralMediaType::Text(_ft) => {
                let contents = fs::read_to_string(file)?;
                let mut firstline = "";
                for l in contents.lines() {
                    if l.trim() != "" {
                        firstline = l;
                        break;
                    }
                }
                Ok((contents.to_string(), firstline.to_string(), "".to_string()))
            }
            _ => Ok(("".to_string(), "".to_string(), "".to_string())),
        }
    }

    fn extract_tika(
        &self,
        tikaconfig: &TikaConfig,
        file: &str,
    ) -> Result<(String, String, String), Box<dyn Error>> {
        let contents = tika::request::text(&tikaconfig, file)?;
        let mut title = "";
        let metatitle = tika::request::title(&tikaconfig, file)?;
        if metatitle != "" {
            title = &metatitle;
        } else {
            for l in contents.lines() {
                if l.trim() != "" {
                    title = l;
                    break;
                }
            }
        }
        Ok((contents.to_string(), title.to_string(), "".to_string()))
    }

    fn get_gmt_string(&self) -> String {
        match self {
            GeneralMediaType::Text(_ft) => "text".to_string(),
            GeneralMediaType::Image(_ft) => "image".to_string(),
            GeneralMediaType::Audio(_ft) => "audio".to_string(),
            GeneralMediaType::Video(_ft) => "video".to_string(),
        }
    }
}

fn get_gmt_from_file(file: &str) -> Result<GeneralMediaType, String> {
    let guess = mime_guess::from_path(file);
    //todo: fix unwrap, crashes on unknown extensions
    if guess.count() == 0 {
        return Err(format!("{} -- Unknown file-extension", file));
    }
    let mimetype = guess.first_raw().unwrap();

    //eprintln!("mime-type: {}", mimetype);
    let mut parts = mimetype.split('/');
    let gmt = parts.next().unwrap();
    let ft = parts.next().unwrap();
    match gmt {
        "text" => Ok(GeneralMediaType::Text(String::from(ft))),
        "application"
            if ft == "vnd.openxmlformats-officedocument.wordprocessingml.document"
                || ft == "vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                || ft == "vnd.openxmlformats-officedocument.presentationml.presentation" =>
        {
            Ok(GeneralMediaType::Text(String::from(ft)))
        }
        "image" => Ok(GeneralMediaType::Image(String::from(ft))),
        "audio" => Ok(GeneralMediaType::Audio(String::from(ft))),
        "video" => Ok(GeneralMediaType::Video(String::from(ft))),
        _ => Err(format!(
            "{} -- Unkown Mediatype {} not implemented",
            file, mimetype
        )),
    }
}

fn get_gmt_from_tika(tikaconfig: &TikaConfig, file: &str) -> Result<GeneralMediaType, String> {
    //eprintln!("tika detect");
    let mimetype = tika::request::detect(&tikaconfig, file).unwrap();
    //eprintln!("mime-type: {}", mimetype);
    let mut parts = mimetype.split('/');
    let gmt = parts.next().unwrap();
    let ft = parts.next().unwrap();
    match gmt {
        "text" => Ok(GeneralMediaType::Text(String::from(ft))),
        "application" => Ok(GeneralMediaType::Text(String::from(ft))),
        "image" => Ok(GeneralMediaType::Image(String::from(ft))),
        "audio" => Ok(GeneralMediaType::Audio(String::from(ft))),
        "video" => Ok(GeneralMediaType::Video(String::from(ft))),
        _ => Err(format!(
            "{} -- Unkown Mediatype {} not implemented.",
            file, mimetype
        )),
    }
}

#[derive(Debug)]
struct Iscc {
    mid: String,
    cid: String,
    did: String,
    iid: String,
    gmt: String,
    title: String,
    extra: String,
    tophash: String,
}

fn get_iscc_id(
    file: &str,
    partial: bool,
    title: &str,
    extra: &str,
    guess: bool,
    tikaconfig: &TikaConfig,
) -> Result<Iscc, Box<dyn Error>> {
    let mediatype = if tikaconfig.active {
        get_gmt_from_tika(tikaconfig, file)?
    } else {
        get_gmt_from_file(file)?
    };

    /*let mut extract = if tikaconfig.active && mediatype.is_tika_extract() {
        mediatype
            .extract_tika(&tikaconfig, &file.to_string())
            .unwrap_or(("".to_string(), "".to_string(), "".to_string()))
    } else {
        mediatype.extract(&file.to_string()).unwrap_or((
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ))
    };*/
    let mut extract = if tikaconfig.active && mediatype.is_tika_extract() {
        mediatype.extract_tika(&tikaconfig, &file.to_string())?
    } else {
        mediatype.extract(&file.to_string())?
    };
    if !guess {
        extract.1 = title.to_string();
        extract.2 = extra.to_string();
    }
    let (extracted_content, extracted_title, extracted_extra) = extract;
    let (mid, metatitle, metaextra) = meta_id(&extracted_title, &extracted_extra);
    let did = data_id(file)?;
    let (iid, tophash) = instance_id(file)?;
    let cid = match &mediatype {
        GeneralMediaType::Text(_ft) => Ok(content_id_text(&extracted_content, partial)),
        GeneralMediaType::Image(_ft) => match content_id_image(file, partial) {
            Ok(id) => Ok(id),
            image_error => Err(format!(
                "Error creating content_id_image: {:?}",
                image_error
            )),
        },
        GeneralMediaType::Audio(_ft) => {
            Err(format!("{}: Mediatype Audio not implemented yet", file))
        }
        GeneralMediaType::Video(_ft) => {
            Err(format!("{}: Mediatype Video not implemented yet", file))
        }
    }?;
    let iscc = Iscc {
        mid,
        cid,
        did,
        iid,
        gmt: mediatype.get_gmt_string(),
        title: metatitle,
        extra: metaextra,
        tophash,
    };
    //eprintln!("{:?}", iscc);
    Ok(iscc)
}
