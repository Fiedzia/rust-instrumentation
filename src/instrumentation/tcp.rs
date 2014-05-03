use std::io::{Listener, Acceptor};
use std::io::net::tcp::{TcpListener};
use std::io::net::ip::{IpAddr, SocketAddr, Port};
use std::result::Result;
use std::from_str::FromStr;
use collections::HashMap;
use self::stream_utils::handle_client;

mod configparse;
mod types;
mod consts;
mod stream_utils;


static MAX_PACKET_SIZE:u32 = 65_535; //2**16-1


pub fn init(config:&HashMap<~str, ~str>,
            command_sender:Sender<types::CommandWithSender>) -> Result<bool, ~str> {
    //! Initialize tcp listener

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
                    let client_stream = client.unwrap();
                    handle_client(client_stream, command_sender_clone, MAX_PACKET_SIZE);
                });
            }
        });
        return Ok(true);
    } else {
        return Ok(false)
    }
}
