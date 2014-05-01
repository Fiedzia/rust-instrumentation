use collections::HashMap;
use std::mem;
use std::io::{Listener, Acceptor, IoError};
use std::io::net::unix::{UnixListener, UnixStream};
use std::io::fs;
use std::io::{Reader, Writer};
use std::result::Result;
use std::vec::Vec;
use std::mem;
use std::io::MemWriter;
use serialize::json;
use std;

use self::stream_utils::{encode_message, decode_message, handle_client};

mod configparse;
mod types;
mod consts;
mod stream_utils;


static MAX_PACKET_SIZE:u32 = 65_535; //2**16-1



/*
 *
 */
pub fn init(config:&HashMap<~str, ~str>, command_sender:Sender<types::CommandWithSender>) -> Result<bool, ~str> {

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
    } else {
        return Ok(false)
    }
    Ok(false)
}
