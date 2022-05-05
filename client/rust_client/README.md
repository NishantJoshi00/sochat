# SWSCA (Secure WebSocket Chat Application)

Simply put here you don't just need someone's id to communicate but also their own generated public key to send them messages


## How to run the client

### Requirements
- Rust [Here!](https://www.rust-lang.org/)
- vcpkg (Might be needed if openssl development is not possible)


# Building
```bash
cargo build --release
```

# Running
```bash
.\\target\\release\\rust_client.exe -c ws://<ip-address>/ws
```
After running the program the config folder will be generated in your current directory
- bug: in the config folder if `connections` folder is not present you will have to manually generate it.
- running the above command again after creating the folder will get the connection established with the server.


