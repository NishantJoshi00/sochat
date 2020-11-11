use clap::{Arg, App};
use websocket::ClientBuilder;
use websocket::Message;

fn input() -> String {
    let mut hold = String::new();
    let _b1 = std::io::stdin().read_line(&mut hold).unwrap();
    hold.to_string()
}

fn main() {
    let version = "0.1.0";
    let matches = App::new("Client for SoChat")
        .version(version)
        .author("Nishant Joshi <nishantjo.12@gmail.com>")
        .author("Give arguments for login and signup information")
        .arg(Arg::with_name("username")
            .short("u")
            .long("user")
            .takes_value(true)
            .value_name("USERNAME")
            .help("username to login with"))
        .arg(Arg::with_name("password")
            .short("p")
            .long("pass")
            .value_name("PASSWORD")
            .takes_value(true)
            .help("Password for login/signup"))
        .arg(Arg::with_name("signup")
            .short("s")
            .long("signup")
            .help("use this flag to login")
        ).get_matches();
        

    let mut app = ClientBuilder::new("ws://localhost:8765")
    .unwrap()
    .connect_insecure()
    .unwrap();
    
    if matches.is_present("signup") {
        app.send_message(&Message::text("signup")).unwrap();
    }
    
}