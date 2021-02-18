use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use clap::{App, Arg};
use pulldown_cmark::{html, Options, Parser};

fn main() -> io::Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("CONTENT")
                .help("content directory")
                .default_value("content")
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("output directory")
                .default_value("out")
                .index(2),
        )
        .get_matches();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);

    let content = matches.value_of("CONTENT").unwrap();
    let content = Path::new(content);

    let output = matches.value_of("OUTPUT").unwrap();
    let output = Path::new(output);

    let files = get_all_files(content)?;
    let mds: Vec<&PathBuf> = files.iter().filter(|f| is_extension(f, "md")).collect();

    for file in &mds {
        let path = file.strip_prefix(content).unwrap();

        let mut file = io::BufReader::new(fs::File::open(file)?);
        let mut md = String::new();
        let _ = file.read_to_string(&mut md).unwrap();
        let parser = Parser::new_ext(&md, options);

        let path = output.join(path);
        let path = path.with_extension("html");
        let parent = path.parent().unwrap();
        if parent.exists() {
            if !parent.is_dir() {
                panic!("parent is not directory");
            }
            println!(
                "parent directory({}) is already exist",
                parent.to_str().unwrap()
            );
        } else {
            fs::create_dir(path.parent().unwrap()).unwrap();
        }
        println!("gen {}", path.to_str().unwrap());

        let mut html = String::new();
        html::push_html(&mut html, parser);

        let file = fs::File::create(path)?;
        let mut file = io::BufWriter::new(file);
        file.write_all(html.as_bytes()).unwrap();

        println!("{}", html);
    }

    Ok(())
}

fn get_all_files(content: &Path) -> io::Result<Vec<PathBuf>> {
    if content.is_dir() {
        let mut v = Vec::new();
        for entry in std::fs::read_dir(content)? {
            let entry = entry?;
            let mut ev = get_all_files(&entry.path())?;
            v.append(&mut ev);
        }
        return Ok(v);
    }

    Ok(vec![content.into()])
}

fn is_extension(p: &PathBuf, ext: &str) -> bool {
    let e = p.extension();
    if e == None {
        return false;
    }
    let e = e.unwrap();
    e == ext
}
