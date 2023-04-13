use clap::{Parser, ValueEnum};
use std::env;
use std::process::exit;
use url::Url;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Output {
    All,
    Emails,
    Links,
    Documents
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    #[arg(short, long)]
    pub url: String,
    #[arg(short, long)]
    pub download: bool,
    #[arg(value_enum, default_value_t = Output::All)]
    pub output: Output,
}

pub fn options() -> CLI {
    let env_args: Vec<String> = env::args().collect();
    if env_args.len() == 2 && env_args[1].contains("http") {
        match Url::parse(&env_args[1]) {
            Ok(_) => {
                return CLI {
                    url: env_args[1].clone(),
                    download: false,
                    output: Output::All,
                }
            }
            Err(_) => return CLI::parse(),
        };
    }
    let args = CLI::parse();
    match Url::parse(&args.url) {
        Ok(_) => {}
        Err(_) => {
            println!("Error parsing invalid URL");
            exit(1);
        }
    }
    args
}

pub fn pipe_list_out(list: Vec<String>) {
    for line in list {
        println!("{}", line);
    }
}
