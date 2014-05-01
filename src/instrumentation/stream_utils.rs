/*
 *  Protocol description:
 *
 *  communication is done via messages,
 *  where client can send one or more messages,
 *  and will receive responses (in the same order)
 *
 *  msg = [ MSG_SIZE, RAW_MSG ]
 *          4 bytes,  MSG_SIZE bytes
 *  MSG_SIZE = 4 bytes unsigned int, big endian
 *  RAW_MSG = MSG_SIZE bytes in utf8
 *  RAW_MSG = COMMAND, [space, COMMAND_PARAM]
 *  COMMAND = "GET_KEY" | "GET_SUBKEYS" | "HAS_KEY"
 *  COMMAND_PARAM = utf8 string
 *  msg examples:
 *
 *  '\x00\x00\x00\x11GET_KEY myapp.foo'
 *
 *  response is similar, except that RAW_MSG will always be json string
 *  all strings are utf8
 *
 */


use collections::HashMap;
use std::mem;
use std::io::{Listener, Acceptor, IoError};
use std::io::net::unix::{UnixListener, UnixStream};
use std::io::{Reader, Writer};
use std::io::fs;
use std::result::Result;
use std::vec::Vec;
use std::mem;
use std::io::MemWriter;
use serialize::json;
use std;

mod configparse;
mod types;
mod consts;



pub fn decode_message(raw_msg:&Vec<u8>)-> Result<(~str, Option<~str>), ~str> {
	  match std::str::from_utf8(raw_msg.as_slice()){
			Some(s) => {
				match configparse::parse_config_line(s.to_owned()) {
					Err(s) => return Err(s),
					Ok(result) => match result {
						configparse::Tokens(tokens) => {
							  if tokens.len() == 0 || tokens.len() > 2 { return Err(~"invalid command")}
								let cmd = tokens.get(0).clone().into_owned();
								if !(cmd.as_slice() == consts::GET_KEY || cmd.as_slice() == consts::HAS_KEY || cmd.as_slice() == consts::GET_SUBKEYS) {
									return Err(~"invalid command")
								}
								if tokens.len() == 1 {
									  if cmd.as_slice() != consts::GET_SUBKEYS {
											return Err(~"invalid command")
										}
										return Ok((cmd, None))
								} else {
									return Ok((cmd, Some(tokens.get(1).clone().into_owned())))
								}

						},
						_ => return Err(~"invalid command")
					}
				}
			},
			None => return Err(~"invalid utf8")
		}
}

pub fn encode_message(msg: json::Json) -> ~Vec<u8> {

	  let raw_bytes:Vec<u8> = Vec::from_slice(msg.to_str().as_bytes());
		let mut memwriter = MemWriter::with_capacity(4+raw_bytes.len());

		memwriter.write_be_u32(raw_bytes.len() as u32);
		memwriter.write(raw_bytes.as_slice());
		~memwriter.unwrap()
}


pub fn handle_client<T: Reader+Writer>(mut client_stream: T, command_sender:Sender<types::CommandWithSender>, max_packet_size:u32) {

    loop {
        let mut bsize:int = 0;
        match client_stream.read_be_u32() {
            Err(e) => fail!(format!("client read failed: {}", e)),
            Ok(packet_size) => {
                if packet_size > max_packet_size { fail!("packet size exceeded")};
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
