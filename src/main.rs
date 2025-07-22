use clap::Parser;
use reqwest::blocking::{get, Client};
use std::error::Error;
use scraper::{Html, ElementRef, Node};
use figlet_rs::FIGfont;
#[derive(Parser)]
#[command(author = "Astral", version = "0.1", about = "CLIb is an experimental browser that runs directly in the terminal!")]

struct Cli {
    #[arg(short, long)]
    website: Option<String>,
    #[arg(short, long)]
    search: Option<String>,
    #[arg(short, long)]
    about: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    if let Some(web) = cli.website {
        let client = Client::new();
        let response = get(web)?
            .text()?;
        let document = Html::parse_document(&response);
        let standart = FIGfont::standard().unwrap();
        for node in document.tree.root().descendants() {
            if let Some(elem) = ElementRef::wrap(node) {
                let tag = elem.value().name();
                let text = &elem.text().collect::<Vec<_>>().join("").trim().to_string();
                if text.is_empty() {
                    continue;
                }
                match tag {
                    "title" => {
                        let big_text = standart.convert(&text).unwrap();
                        println!("{}", big_text);
                    }
                    "h1" => {
                        println!("\x1b[1m{}\x1b[0m", text);
                    }
                    "p" => {
                        println!("{}", text);
                    }
                    _ => continue,
                };
                
            }

            
        }
    }
    if cli.about == true {
        println!("\x1b[1mCLI Browser\x1b[0m – use only the terminal to surf the internet!\nRight now you can \x1b[1mbrowse websites\x1b[0m and read HTML pages.\n\x1b[1mComing soon:\x1b[1m\nSearch via Google, Yandex, DuckDuckGo\n");
    }
    if let Some(web) = cli.search {
        let client = Client::new();
        let prompt = web.replace(" ", "+");
        let url = format!("https://duckduckgo.com/html/?q={}", prompt);
        let response = client
                .get(url)
                     .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
                         .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
                             .header("Accept-Language", "ru-RU,ru;q=0.9,en-US;q=0.8,en;q=0.7")
                                 .header("Accept-Encoding", "gzip, deflate, br")
                                     .header("Connection", "keep-alive")
                                         .header("Upgrade-Insecure-Requests", "1")
                    .send()?
                    .text()?;
        let document = Html::parse_document(&response);
        let standart = FIGfont::standard().unwrap();

        for node in document.tree.root().descendants() {
                if let Some(elem) = ElementRef::wrap(node) {
                            let tag = elem.value().name();
                                    let text = &elem.text().collect::<Vec<_>>().join("").trim().to_string();
                                            if text.is_empty() {
                                                            continue;
                                                       }
                                                    match tag {
                                                    "title" => {
                                                        let big_text = standart.convert(&text).unwrap();
                                                        println!("{}", big_text);
                                                    }
                                                    "h1" => {
                                                        println!("\x1b[1m{}\x1b[0m", text);
                                                    }
                                                    "p" => {
                                                        println!("{}", text);
                                                    }
                                                    _ => continue,
                                        
                                    }
                }
                        
                            
                }
            
        }
    Ok(())

}
