use websocket::{ClientBuilder};
// use websocket::stream::async::AsyncStream;


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


fn encode_v_s(value: Vec<u8>) -> String {
    let mut encoded = String::new();
    for i in value {
        encoded.push_str(&format!("{:02x}", i));
    }
    encoded
}

fn decode_s_v(value: String) -> Vec<u8> {
    let mut decoded = Vec::new();
    let mut hex = String::new();
    for i in value.chars() {
        hex.push(i);
        if hex.len() == 2 {
            decoded.push(u8::from_str_radix(&hex, 16).unwrap());
            hex.clear();
        }
    }
    decoded
}

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
        // let client = ClientBuilder::new(&url)?.connect_secure(None)?;


        let (mut reciever, mut sender) = client.split()?;
        

        
        // Connection established




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




        println!("Logged in as {}#{}", user.name, user.id);



        // Event loops
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
                            println!("{}: {}", message.from.bold().yellow(), message.data);
                        } else {
                            let data_t_e = decode_s_v(message.data);
                            let mut a = 256;
                            while data_t_e.len() > a {
                                a = a << 1;
                            }
                        
                            let mut decrypted: Vec<u8> = vec![0; a];
                            let _ = private_key.private_decrypt(&data_t_e, &mut decrypted, openssl::rsa::Padding::PKCS1).unwrap();
                            println!("{}: {}", message.from.bold().green(), String::from_utf8(decrypted).unwrap());
                        }
                    }
                    _ => ()
                }
            }
        });















        // user interaction logic
        let mut indicator = String::from("");
        let mut public_key: Option<openssl::rsa::Rsa<openssl::pkey::Public>> = None;
        loop {
            // println!("{} ", &indicator);
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            let finput = input.trim();
            let message = match finput {
                "?help" => {
                    print_help();
                    continue;
                },
                "?online" => {
                    let send = WsMessage {
                        encrypted: false,
                        data: format!("/online {}#{}", &user.name, &user.id),
                        from: format!("{}#{}", &user.name, &user.id),
                        to: "server".to_owned()
                    };
                    OwnedMessage::Text(serde_json::to_string(&send).unwrap())
                }
                "?refresh" => {
                    let paths = fs::read_dir("./config/connections/").unwrap();
                    let mut users = String::new();  
                    users.push_str("/active ");
                    for path in paths {
                        let p = path.unwrap();
                        users.push_str(p.path().file_name().unwrap().to_str().unwrap().split(".").into_iter().next().unwrap());
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
                        let paths = fs::read_dir("./config/connections/").unwrap();
                        for path in paths {
                            let p = path.unwrap();
                            let f_name = p.path().file_name().unwrap().to_str().unwrap().to_string();
                            if startswith(&f_name, &name.to_owned()) {
                                current.push(f_name.split(".").into_iter().next().unwrap().to_string());
                            }
                        }
                        if current.len() > 1 {
                            println!("{} names found: ", current.len());
                            for name in &current {
                                println!("{}, ", name);
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
                        println!("Connected to /{}", &indicator.green().bold());
                        public_key = Some(Rsa::public_key_from_pem(fs::read(format!("./config/connections/{}.pem", indicator)).unwrap().as_slice()).unwrap());
                        continue;
                    } else {
                        if indicator == "" {
                            println!("{}: choose a sender", "Error".red().bold());
                            continue;
                            
                        } else {
                            let buf = match &public_key {
                                Some(pky) => {
                                    let data_t_e: Vec<u8> = String::from(val).into_bytes();
                                    let mut a = 256;
                                    while data_t_e.len() > a {
                                        a = a << 1;
                                    }
                                
                                    let mut buf: Vec<u8> = vec![0; a];
                                    
                                    pky.public_encrypt(val.as_bytes(), buf.as_mut_slice(), openssl::rsa::Padding::PKCS1).unwrap();
                                    buf
                                },
                                None => {
                                    println!("{}: Something went wrong while loading the key", "Error".red().bold());
                                    println!("{}: Please select a user again to continue", "Hint".yellow().bold());
                                    continue;
                                }
                            };
                            let message_to_send = encode_v_s(buf);
                            let send = WsMessage {
                                encrypted: true,
                                data: message_to_send,
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