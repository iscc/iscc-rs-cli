# iscc-rs-cli
ISCC cli for the iscc-rs library (https://github.com/iscc/iscc-rs) 0.1

## Usage:
```
    iscc-cli [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
        --help       Prints help information
    -k, --tika       Use Apache Tika for media-type detection and text-extraction
    -v               Sets the level of verbosity
    -V, --version    Prints version information

OPTIONS:
    -h, --host <TIKAHOST>    Hostname or Ipaddress of an Apache Tika server (default: localhost)
    -p, --port <PORT>        Port of a Apache Tika server (default: 9998)

SUBCOMMANDS:
    batch    Create ISCC Codes for all files in PATH.
    gen      Generate ISCC Code for FILE.
    help     Prints this message or the help of the given subcommand(s)

```    
    
## Supported formats in standalone mode:
* text
* docx
* xlsx
* gif
* png

## Supported formats using Apache Tika:
https://tika.apache.org/1.23/formats.html

