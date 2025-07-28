use reqwest::blocking::{get, Client};
use std::error::Error;
use scraper::{Html, ElementRef};
use figlet_rs::FIGfont;
use std::io::{Write, stdout};

pub fn render(web: String, _search: bool, ) -> Result<(), Box<dyn Error>>  { 
    let mut document: Html = Html::parse_document("");
    let mut standart = FIGfont::standard().unwrap();
    match _search {
    true => {
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
                    document = Html::parse_document(&response);
        }
        false => {
            let _client = Client::new();
                    let response = get(web)?
                        .text()?;
                    document = Html::parse_document(&response);
               
                      
             
             
        }
    };
    stdout().flush().unwrap();
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
                                                            println!("\x1b[7m{}\x1b[0m", text);
                                                        }
                                                        "h2" => {
                                                        	println!("\x1b[1m{}\x1b[0m", text);
                                                        }
                                                        "h3" => {
                                                        	println!("\x1b[1m{}\x1b[0m", text);
                                                        }
                                                        "h4" => {
                                                             println!("\x1b[1m{}\x1b[0m", text);
                                                        }
                                                        "h5" => {
                                                        	println!("\x1b[1m{}\x1b[0m", text);
                                                        }
                                                        "h6" => {
                                                        	println!("\x1b[1m{}\x1b[0m", text);
                                                        }
                                                        "p" => {
                                                            println!("{}", text);
                                                        }
                                                        _ => continue,
                                            
                                        }
                    }
                            
                                
                    }
    Ok(())
    }
