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

use self::stream_utils::{encode_message, decode_message};

mod configparse;
mod types;
mod consts;
mod stream_utils;


static MAX_PACKET_SIZE:u32 = 65_535; //2**16-1

fn handle_client(mut client_stream: UnixStream, command_sender:Sender<types::CommandWithSender>) {

    loop {
        let mut bsize:int = 0;
        match client_stream.read_be_u32() {
            Err(e) => fail!(format!("client read failed: {}", e)),
            Ok(packet_size) => {
                if packet_size > MAX_PACKET_SIZE { fail!("packet size exceeded")};
                let bytes = client_stream.read_exact(packet_size as uint);
                match bytes {
                    Ok(bytes) => {
                        match decode_message(&bytes){
                            Err(e) => fail!(format!("Failed to decode message: {}", e)),
                            Ok((cmd, key)) => {
                              let response_channel:types::CommandResponseChannel = channel();
                              let (response_sender, response_receiver) = response_channel;
                              let x : types::CommandWithSender = (response_sender, (cmd, key));
                              command_sender.send(x);
                              let x = response_receiver.recv();
                              client_stream.write(encode_message(x).as_slice());
                            }
                        }
                    },
                    Err(e) => { return}
                }
            }
        }
    }
}



/*
 *
 */
pub fn init(config:HashMap<~str, ~str>, command_sender:Sender<types::CommandWithSender>) -> Result<bool, ~str> {

    if config.contains_key(&~"socket.unix") && config.get(&~"socket.unix") == &~"on" {
        let path = config.get(&~"socket.unix.path").to_owned();
        spawn(proc(){
            let socket_path = Path::new(path);
            fs::unlink(&socket_path);
            let stream = UnixListener::bind(&socket_path);
            info!("starting unix socket listener on {}", (socket_path.as_str()));
            for client in stream.listen().incoming() {
                
                let command_sender_clone = command_sender.clone();
                spawn(proc(){
                    let mut client_stream = client.unwrap();
                    handle_client(client_stream, command_sender_clone);
                });
            }
        });
    } else {
        return Ok(false)
    }
    Ok(false)
}
