use websocket::{ClientBuilder};

use std::io::stdin;
use std::sync::mpsc::channel;
use serde_json;
// use std::env;
use std::error::Error;
use std::fs;

use clap::{Arg, App};

mod packets;
use packets::{init_config, User, WsMessage};
use websocket::OwnedMessage;
use openssl::rsa::Rsa;
use colored::*;
fn main() -> Result<(), Box<dyn Error>> {

    // CLI Startup
    let matches = App::new("Secure Chat")
        .version("1.0.0")
        .author("Nishant J. <nishantjo.12@gmail.com>")
        .about("Chat with dumb people")
        .arg(Arg::with_name("connect").short("c").long("connect").value_name("<URL>").takes_value(true))
        .get_matches();

    let url: String;
    if let Some(uri) = matches.value_of("connect") {
        url = String::from(uri);
    } else {
        println!("No URL provided!");
        return Ok(());
    }
    // Key store
    println!("Forward!");

    if std::path::Path::new("./config/private_key.pem").exists() && std::path::Path::new("./config/user.cfg").exists() {
        let contents = fs::read("./config/private_key.pem")?;
        let private_key = Rsa::private_key_from_pem(contents.as_slice())?;
        let mut user: User = serde_json::from_slice(fs::read("./config/user.cfg")?.as_slice())?;
        let client = ClientBuilder::new(&url)?.connect_insecure()?;
        // Connection established
        let (mut reciever, mut sender) = client.split()?;
        let (tx, rx) = channel();
        let _ = sender.send_message(&OwnedMessage::Text(serde_json::to_string(&user)?));

        let msg = reciever.recv_message()?;
        match msg {
            OwnedMessage::Close(_) => return Ok(()),
            OwnedMessage::Text(t) => {
                user = serde_json::from_str(t.as_str())?;
                std::fs::write("./config/user.cfg", serde_json::to_string(&user).unwrap())?;
            },
            _ => ()
        }; 

        println!("User: {}", user);
        let tx_1 = tx.clone();

        let sender_loop = std::thread::spawn(move || {
           loop {
               let message = match rx.recv() {
                   Ok(m) => m,
                   Err(e) => {
                       println!("{} in sender: {:?}", "Error".red().bold(), e);
                       return;
                   }
               };
               match message {
                   OwnedMessage::Close(_) => {
                       let _ = sender.send_message(&message);
                   }
                   _ => (),
               }
               match sender.send_message(&message) {
                   Ok(()) => (),
                   Err(e) => {
                       println!("{} in sender: {:?}", "Error".red().bold(), e);
                       return;
                   }
               }
           } 
        });


        let receive_loop = std::thread::spawn(move || {
            for message in reciever.incoming_messages() {
                let message = match message {
                    Ok(m) => m,
                    Err(e) => {
                        println!("{} in reciever: {:?}", "Error".red().bold(), e);
                        let _ = tx_1.send(OwnedMessage::Close(None));
                        return;
                    }
                };

                match message {
                    OwnedMessage::Close(_) => {
                        let _ = tx_1.send(OwnedMessage::Close(None));
                        return;
                    }
                    OwnedMessage::Ping(data) => {
                        match tx_1.send(OwnedMessage::Pong(data)) {
                            Ok(()) => (),
                            Err(e) => {
                                println!("{} in reciever: {:?}", "Error".red().bold(), e);
                                return;
                            }
                        }
                    }
                    OwnedMessage::Text(msg) => {
                        let message: WsMessage = serde_json::from_str(msg.as_str()).unwrap();
                        if !message.encrypted {
                            println!("{}: {}", message.from, message.data);
                        } else {
                            let mut decrypted: Vec<u8> = Vec::new();
                            let _ = private_key.private_decrypt(message.data.as_bytes(), &mut decrypted, openssl::rsa::Padding::PKCS1).unwrap();
                            println!("{}: {}", message.from, String::from_utf8(decrypted).unwrap());
                        }
                    }
                    _ => ()
                }
            }
        });
        let mut indicator = String::from("");
        let mut public_key: Option<openssl::rsa::Rsa<openssl::pkey::Public>> = None;
        loop {
            print!("{} ", &indicator);
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            let finput = input.trim();
            let message = match finput {
                "?help" => {
                    print_help();
                    continue;
                },
                "?refresh" => {
                    let paths = fs::read_dir("./config/con/").unwrap();
                    let mut users = String::new(); 
                    users.push_str("/active ");
                    for path in paths {
                        let p = path.unwrap();
                        users.push_str(p.path().file_name().unwrap().to_str().unwrap());
                        users.push_str(" ");
                    }

                    let send = WsMessage {
                        encrypted: false,
                        data: users,
                        from: format!("{}#{}", &user.name, &user.id),
                        to: "server".to_owned()
                    };
                    OwnedMessage::Text(serde_json::to_string(&send).unwrap())
                },
                "?exit" => {
                    let _ = tx.send(OwnedMessage::Close(None));
                    break;
                },
                val => {
                    if val.chars().nth(0).unwrap() == '/' {
                        let mut name = val.chars();
                        name.next();
                        let name = name.as_str();
                        let mut current: Vec<String> = Vec::new();
                        let paths = fs::read_dir("./config/con/").unwrap();
                        for path in paths {
                            let p = path.unwrap();
                            let f_name = p.path().file_name().unwrap().to_str().unwrap().to_string();
                            if startswith(&f_name, &name.to_owned()) {
                                current.push(f_name.clone());
                            }
                        }
                        if current.len() > 1 {
                            print!("{} names found: ", current.len());
                            for name in &current {
                                print!("{}, ", name);
                            }
                            println!("");
                            continue;
                        }
                        if current.len() == 0 {
                            println!("{}: The pattern didn't match to any users stored", "Error".red().bold());
                            continue;
                        }
                        let to_name = current.pop().unwrap();
                        indicator = to_name.clone();
                        public_key = Some(Rsa::public_key_from_pem(fs::read(format!("./config/con/{}.pem", indicator)).unwrap().as_slice()).unwrap());
                        continue;
                    } else {
                        if indicator == "" {
                            println!("{}: choose a sender", "Error".red().bold());
                            continue;
                            
                        } else {

                            let mut buf: Vec<u8> = Vec::new();
                            let _ = match &public_key {
                                Some(pky) => pky.public_encrypt(val.as_bytes(), &mut buf, openssl::rsa::Padding::PKCS1).unwrap(),
                                None => {
                                    println!("{}: Something went wrong while loading the key", "Error".red().bold());
                                    println!("{}: Please select a user again to continue", "Hint".yellow().bold());
                                    continue;
                                }
                            };
                            let send = WsMessage {
                                encrypted: true,
                                data: std::str::from_utf8(&buf).unwrap().to_owned(),
                                from: format!("{}#{}",user.name, user.id),
                                to: indicator.to_owned()
                            };
                            OwnedMessage::Text(serde_json::to_string(&send).unwrap())
                        }
                    }
                }
            };

            match tx.send(message) {
                Ok(_) => (),
                Err(e) => {
                    println!("{} in client engine: {:?}", "Error".red().bold(), e);
                }
            }
        }

        let _ = sender_loop.join();
        let _ = receive_loop.join();

    } else {
        init_config()?;
        return Ok(());
    }
    Ok(())
}

fn print_help() {
    println!("{}", "Help".white().bold());
    println!("Use {} for ops, {} for changing channels", "?".bold(), "/".bold());
    println!("{}\t: To perform system functionality", "Commands".white().bold());
    println!("{}\t: to show this prompt", "?help".yellow().bold());
    println!("{}\t: To refresh list from the server", "?refresh".yellow().bold());
    println!("Only possbile if he has your public key and tag");
    println!("{}\t: To exit the conversation with the server", "?exit".yellow().bold());

    
}

fn startswith(first: &String, second: &String) -> bool {
    let mut f = first.chars();
    for s in second.chars() {
        let b = match f.next() {
            Some(v) => v,
            None => {
                return false;
            }
        };
        if s == b {
            continue;
        } else {
            return false;
        }
    }
    true
}