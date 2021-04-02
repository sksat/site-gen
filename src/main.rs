use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use clap::{App, Arg};
use pulldown_cmark::{html, Options, Parser};
use writedown_html;
use writedown_html::writedown::Render;

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

    let content = matches.value_of("CONTENT").unwrap();
    let content = Path::new(content);

    let output = matches.value_of("OUTPUT").unwrap();
    let output = Path::new(output);

    let files = get_all_files(content)?;
    for f in &files {
        let ext = f.extension();
        let path = f.strip_prefix(content).unwrap();

        let mut file = io::BufReader::new(fs::File::open(f)?);
        let mut src = String::new();
        let _ = file.read_to_string(&mut src).unwrap();

        let path = output.join(path);
        let parent = path.parent().unwrap();
        if parent.exists() {
            if !parent.is_dir() {
                panic!("parent is not directory");
            }
            //println!(
            //    "parent directory({}) is already exist",
            //    parent.to_str().unwrap()
            //);
        } else {
            fs::create_dir_all(path.parent().unwrap()).unwrap();
        }
        print!("{} ...\t", f.to_str().unwrap());

        let mut out = String::new();

        if ext.is_none() {
            panic!("no extension")
        }
        let ext = ext.unwrap().to_str().unwrap();
        let path = match ext {
            "html" | "png" | "jpg" => {
                //println!("{} -> {}", f.to_str().unwrap(), path.to_str().unwrap());
                let _ = fs::copy(f, path).unwrap();
                println!("[copy]");
                continue;
            }
            "md" => {
                let mut options = Options::empty();
                options.insert(Options::ENABLE_STRIKETHROUGH);
                options.insert(Options::ENABLE_FOOTNOTES);

                let parser = Parser::new_ext(&src, options);
                html::push_html(&mut out, parser);
                path.with_extension("html")
            }
            "wd" => {
                let html = writedown_html::from_str(&src).unwrap();
                out = html.render();
                path.with_extension("html")
            }
            _ => {
                println!("[skip]");
                continue;
            }
        };

        // generate HTML header
        if path.extension().unwrap().to_str().unwrap() == "html" {
            out = if out.starts_with("<!DOCTYPE>") || out.starts_with("<html>") {
                out
            } else {
                let title = if out.starts_with("<h1>") {
                    let body = out.strip_prefix("<h1>").unwrap();
                    let end = body.find("</h1>").unwrap();
                    &body[..end]
                } else {
                    "default title"
                };
                let title = if title.len() == 0 {
                    "".to_string()
                } else {
                    format!("  <title>{}</title>\n", title)
                };

                let head0 = concat!(
                    "<!DOCTYPE html>\n",
                    "<html>\n",
                    "<head>\n",
                    "  <meta charset=\"UTF-8\">\n",
                    "  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n"
                );
                let head1 = concat!("</head>\n");
                let header = format!("{}{}{}", head0, title, head1);
                let footer = concat!("\n</html>");

                format!("{}<body>\n{}\n</body>{}", header, out, footer)
            };
        }

        if out.len() == 0 {
            println!("empty");
            continue;
        }

        let file = fs::File::create(path)?;
        let mut file = io::BufWriter::new(file);
        file.write_all(out.as_bytes()).unwrap();
        println!("[ok]");
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
