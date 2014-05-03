use std::io::{Listener, Acceptor};
use std::io::net::unix::{UnixListener};
use std::io::fs;
use std::result::Result;
use collections::HashMap;
use self::stream_utils::{handle_client};

mod configparse;
mod types;
mod consts;
mod stream_utils;


static MAX_PACKET_SIZE:u32 = 65_535; //2**16-1


pub fn init(config:&HashMap<~str, ~str>,
            command_sender:Sender<types::CommandWithSender>) -> Result<bool, ~str> {
    //! Initialize unix socket listener

    if config.contains_key(&~"socket.unix") && config.get(&~"socket.unix") == &~"on" {
        let path = config.get(&~"socket.unix.path").to_owned();
        spawn(proc(){
            let socket_path = Path::new(path);
            fs::unlink(&socket_path);
            let listener = UnixListener::bind(&socket_path);
            info!("starting unix socket listener on {}", (socket_path.as_str()));
            for client in listener.listen().incoming() {
                
                let command_sender_clone = command_sender.clone();
                spawn(proc(){
                    let mut client_stream = client.unwrap();
                    handle_client(client_stream, command_sender_clone, MAX_PACKET_SIZE);
                });
                
            }
        });
        return Ok(true);
    } else {
        return Ok(false)
    }
    Ok(false)
}
