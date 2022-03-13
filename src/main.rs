mod util;

use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
    process::exit,
    str::FromStr,
};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    name = "git-crendential-helper(pass)",
    about = "A customer cache for git crendential",
    author,
    version
)]
struct Opt {
    #[clap(
        short = 'f',
        long = "file",
        help = "the file path which will be used to keep your passwords"
    )]
    filename: Option<String>,
    #[clap(name = "subcommand", subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[clap(name = "get")]
    Get,
    #[clap(name = "store")]
    Store,
    #[clap(name = "erase")]
    Erase,
}

#[derive(Debug)]
struct Credential {
    prot: String,
    user: String,
    pass: String,
    host: String,
}

impl Credential {
    pub fn new<T>(prot: T, user: T, pass: T, host: T) -> Self
    where
        T: ToString,
    {
        Self {
            prot: prot.to_string(),
            user: user.to_string(),
            pass: pass.to_string(),
            host: host.to_string(),
        }
    }
}

fn main() -> io::Result<()> {
    let opt = Opt::parse();

    let filename = opt.filename.map(PathBuf::from).unwrap_or(
        home::home_dir()
            .unwrap()
            .as_path()
            .join(".git-crendential-pass-store"),
    );

    match opt.command {
        Command::Get => {
            let mut known_hosts: HashMap<String, String> = HashMap::new();
            // waiting for user input
            loop {
                let mut s = String::new();
                let _ = io::stdin().read_line(&mut s)?;
                if s.trim().is_empty() {
                    break;
                }
                if let Some((k, v)) = s.trim().split_once('=') {
                    known_hosts.insert(k.trim().to_owned(), v.trim().to_owned());
                }
            }

            // find the first matching record
            for line in BufReader::new(File::open(filename)?).lines() {
                let line = line?;
                let re = regex::Regex::new("^(.*?)://(.*?):(.*?)@(.*)$").unwrap();
                let res = re.captures(&line);
                if res.is_none() {
                    continue;
                }
                let res = res.unwrap();
                let cred = Credential::new(
                    res.get(1).unwrap().as_str(),
                    res.get(2).unwrap().as_str(),
                    res.get(3).unwrap().as_str(),
                    res.get(4).unwrap().as_str(),
                );
                // 比较
                if cred.prot == known_hosts["protocol"] && cred.host == known_hosts["host"]
                // && cred.user == known_hosts["username"]
                {
                    println!("protocal={}", cred.prot);
                    println!("host={}", cred.host);
                    println!("username={}", cred.user);
                    println!("password={}", cred.pass);
                    exit(0);
                }
            }
        }
        Command::Store => {}
        Command::Erase => {}
    }
    Ok(())
}

#[test]
fn test_regex() {
    let s = "https://bob:s3cre7@mygithost";
    let re = regex::Regex::new("^(.*?)://(.*?):(.*?)@(.*)$").unwrap();
    let res = re.captures(s);
    assert!(res.is_some());
}
