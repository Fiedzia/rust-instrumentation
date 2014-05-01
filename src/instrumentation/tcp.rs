use collections::HashMap;
use std::mem;
use std::io::{Listener, Acceptor, IoError};
use std::io::net::tcp::{TcpListener, TcpStream};
use std::io::net::ip::{IpAddr, Ipv4Addr, SocketAddr, Port};
use std::io::fs;
use std::io::{Reader, Writer};
use std::result::Result;
use std::vec::Vec;
use std::mem;
use std::io::MemWriter;
use serialize::json;
use std::from_str::FromStr;
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

    if config.contains_key(&~"socket.tcp") && config.get(&~"socket.tcp") == &~"on" {
        let port:Port = FromStr::from_str(config.get(&~"socket.tcp.port").to_owned()).unwrap();
				let ipaddr:IpAddr = FromStr::from_str(config.get(&~"socket.tcp.addr").to_owned()).unwrap();

        spawn(proc(){

					  let addr = SocketAddr { ip: ipaddr, port: port };
					  let listener = TcpListener::bind(addr);

            info!("starting tcp socket listener on {}:{}", ipaddr.to_str(), port.to_str());
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
