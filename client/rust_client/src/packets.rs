use openssl;
use serde::{Serialize, Deserialize};
use serde_json;
use std::error::Error;
use std::{fs, io};
use std::io::{Write};
use std::fmt::{Display, Formatter};
// use rand;


use colored::*;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub token: String
}


#[derive(Serialize, Deserialize)]
pub struct WsMessage {
    pub encrypted: bool,
    pub data: String,
    pub from: String,
    pub to: String
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {

        write!(f, "id={} name={} token={}", self.id, self.name, self.token)
    }
}

pub fn init_config() -> Result<(), Box<dyn Error>> {
    // Creating a new pair of keys
    println!("{}", "Configuration not found, creating new profile".white().bold());
    
    let private_key = openssl::rsa::Rsa::generate(2048).unwrap();
    println!("{} {}: {}", "[1/3]".truecolor(200, 200, 200).bold(), "Creating RSA:2048 key pair", "Done".green().bold());
    let _if = fs::create_dir("./config");
    let _if = fs::create_dir("./config/con");
    std::fs::write("./config/private_key.pem", private_key.private_key_to_pem().unwrap())?;
    std::fs::write("./config/public_key.pem", private_key.public_key_to_pem().unwrap())?;
    println!("{} {}: {}", "[2/3]".truecolor(200, 200, 200).bold(), "Saving keys to files", "Done".green().bold());
    // Key pair created and saved in ./config

    println!("{} {}: {}", "[3/3]".truecolor(200, 200, 200).bold(), "Creating RSA:2048 Key Pair", "Started...".yellow().bold());
    let mut name = String::new();
    print!("Enter your name: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut name).expect("Failed to read the input");
    println!();
    trim_newline(&mut name);
    let name = name.clone();
    let id = "-1".to_owned();
    let mut token = String::new();
    print!("Enter the access token: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut token).expect("Failed to read the input");
    trim_newline(&mut token);
    let token = token.clone();

    let user = User {
        id, name, token
    };

    std::fs::write("./config/user.cfg", serde_json::to_string(&user).unwrap())?;
    println!("{}: {}", "Done".bold(), "Please re-run the application to communicate with the server");
    Ok(())
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}