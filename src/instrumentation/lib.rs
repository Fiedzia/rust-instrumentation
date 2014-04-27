#![crate_id = "instrumentation#0.1"]
#![crate_type = "lib"]

extern crate collections;
extern crate serialize;
extern crate sync;

use std::comm::{Sender, Receiver};
use std::default::Default;
use std::os::getenv;
use std::path;
use collections::HashMap;
use std::result::Result;
use std::comm::Select;
use std::io::MemWriter;
use serialize::{json, Encodable};



pub use types::Instrument;

mod configparse;
mod unix;
mod types;


pub static ENV_VAR:  & 'static str = "INSTRUMENTATION";


//obtain configuration
pub fn get_config() -> Result<HashMap<~str, ~str>, ~str> {
    
    let env_var = getenv(ENV_VAR);
    if !env_var.is_some() { return Ok(HashMap::new()) }
    let env_var_value:~str = env_var.unwrap().clone();
    let path = path::Path::new(env_var_value.clone());
		let lines = match path.exists() {
			true  => configparse::read_file_config(path),
			false => configparse::read_env_var_config(env_var_value)
		};
		configparse::parse_config_lines(lines)
}

fn dotsplit(s:~str) -> (~str, Option<~str>) {
		match s.find_str(&".") {
			None => (s, None),
			Some(idx) => (s.slice(0, idx).to_owned(), Some(s.slice(idx+1, s.len()).to_owned()))
		}
}

/* 
 *
 *
 *
*/
fn start (receiver:Receiver<Instrument>, command_sender: Sender<types::CommandWithSender>, command_receiver: Receiver<types::CommandWithSender>){



    let config_result = get_config();
		match config_result {
			Ok(config) => { unix::init(config,  command_sender.clone());},
			Err(errstr) => fail!(errstr)
		}
    //parse config
    //set up listener tasks
		
		let mut instruments: HashMap<~str, Instrument> = HashMap::new();

		let sel = Select::new();
		let mut instrument_receiver_handle = sel.handle(&receiver);
		let mut command_handle = sel.handle(&command_receiver);
		unsafe { command_handle.add(); instrument_receiver_handle.add(); }
		
		loop {

			  let ret = sel.wait();
				if ret == instrument_receiver_handle.id() {
			      //handle case when
  		      //sender channels goes out of scope
      		  //causing receiver to be closed by rust
						let instrument = instrument_receiver_handle.recv_opt();
						if !instrument.is_ok(){
								unsafe{ instrument_receiver_handle.remove(); }
								continue;
						}
						let n:~str = instrument.unwrap().name.clone().to_owned();
						instruments.insert(n, instrument.unwrap());
					  
				} else if ret == command_handle.id() {
						let data = command_handle.recv_opt();
						if !data.is_ok() { continue; }
						let (cmd_response_sender, request) = data.unwrap();
						let (cmd, param) = request;
						//handle top-level requests
						if param.is_none() {
							let result:Option<~str> = if cmd == ~"GET_KEY" { None }
							else if cmd == ~"HAS_KEY" { None}
							else if cmd == ~"GET_SUBKEYS" {
								let keys: ~[~str] = instruments.keys().map(|k| k.to_owned()).collect();
								//let s:~str = json::List(keys);
								let s:~str = json::Encoder::str_encode(&keys);
								Some(s) 
							}
							else { None };
							  
							cmd_response_sender.send(result);
						} else {
							let (first, rest) = dotsplit(param.unwrap());
							if !instruments.contains_key(&first) {
									cmd_response_sender.send(None);
							} else if rest.is_none() && first != ~"GET_SUBKEYS" {
									cmd_response_sender.send(None);
							} else {
									let inst = instruments.get(&first);
									let result = if cmd == ~"GET_KEY" { let f = inst.get_key(rest.unwrap()); println!("f::{}", f); f }
									else {None };
									cmd_response_sender.send(result);
									/*cmd_response_sender.send(
											match(cmd) {
												~"GET_KEY" => inst.get_key(rest),
												~"HAS_KEY" => inst.has_key(rest),
												~"GET_SUBKEYS" => inst.get_subkeys(rest),
												_ => None
											}
									);*/
							}
							//}

						}
						
				} else {fail!("select?") }
				
		}

    /*loop {
        let instrument = receiver.recv_opt();
				println!("got instrument");
				let data = command_receiver.recv();
				let (cmd_response_sender, cmd) = data;
				cmd_response_sender.send(Some(~"7"));

        //handle case when
        //sender channels goes out of scope
        //causing receiver to be closed by rust
        if !instrument.is_ok() { break; }
				let n:~str = instrument.unwrap().name.clone().to_owned();
				//select

    }*/
}



/* Global instrumentation initialization
 * Spawns instrumentation task
 * and returns sender accepting
 * Instrument instances.
 * TODO: change result to option for
 * better error handling, make it optionally synchronous
*/
pub fn init() -> Sender<Instrument>{
    let instrumentation_channel:types::InstrumentationChannel = channel();
    let (sender, receiver) = instrumentation_channel;
		let command_channel:types::CommandChannel = channel();
		let (command_sender, command_receiver) = command_channel;
    spawn(proc() {
        start(receiver, command_sender, command_receiver)
    });
    sender
}

//register

pub fn register(sender:Sender<Instrument>, i:Instrument) {

    sender.send(i);
}

//set up listeners
//set up exporters
