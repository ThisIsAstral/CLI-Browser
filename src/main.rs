pub mod html_render;

use html_render::{render};
use clap::Parser;
use reqwest::blocking::Client;
use std::error::Error;
use std::io::{self, Write, stdin, stdout};
use scraper::{Html, ElementRef, Node};
use figlet_rs::FIGfont;
use std::panic;
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
       render(web, false);
   
       
       
       	
      

        
    }
    if cli.about == true {
        println!("\x1b[1mCLI Browser\x1b[0m – use only the terminal to surf the internet!\nRight now you can \x1b[1mbrowse websites\x1b[0m and read HTML pages.\n\x1b[1mComing soon:\x1b[1m\nSearch via Google, Yandex, DuckDuckGo\n");
    }
    if let Some(web) = cli.search {
        render(web, true);
        print!("\x1B[?1049l");
        }
    Ok(())

}
