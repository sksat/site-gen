use clap::{App, Arg};
use pulldown_cmark::{html, Options, Parser};
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("FILE")
                .help("markdown file")
                .required(true)
                .index(1),
        )
        .get_matches();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);

    let file = matches.value_of("FILE").unwrap();
    let mut file = BufReader::new(File::open(file).unwrap());
    let mut md = String::new();
    let _ = file.read_to_string(&mut md).unwrap();

    let parser = Parser::new_ext(&md, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    println!("{}", html_output);
}
