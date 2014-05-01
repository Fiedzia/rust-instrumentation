#![crate_id = "instrumentation#0.1"]
#![crate_type = "lib"]

#![feature(phase)]
#[phase(syntax, link)] extern crate log;


extern crate collections;
extern crate serialize;
extern crate sync;
extern crate serialize;


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
pub use consts::{CONFIG_FILE, ENV_VAR};

mod configparse;
mod unix;
mod types;
mod consts;


/// obtain configuration
/// trying, in order:
/// INSTRUMENTATION variable (which can contain filename or configuration itself)
/// instrumentation.conf file, if present
/// return empty HashMap if no configuration is present
pub fn get_config() -> Result<HashMap<~str, ~str>, ~str> {
    
    let env_var = getenv(ENV_VAR);
    if !env_var.is_some() {
        let path = path::Path::new(CONFIG_FILE);
        match path.exists(){
            true => return configparse::parse_config_lines(configparse::read_file_config(path)),
            false => return Ok(HashMap::new())
        }
    }
    let env_var_value:~str = env_var.unwrap().clone();
    let path = path::Path::new(env_var_value.clone());
    let lines = match path.exists() {
      true  => configparse::read_file_config(path),
      false => configparse::read_env_var_config(env_var_value)
    };
    configparse::parse_config_lines(lines)
}

/// Split string s into two parts, separated by first . character.
/// This functions assumes that s is not empty
/// ie. ~"foo.bar" -> ~"foo", Some(~"bar")
/// ~"foo" -> ~"foo", None
fn dotsplit(s:~str) -> (~str, Option<~str>) {
    match s.find_str(&".") {
      None => (s, None),
      Some(idx) => (s.slice(0, idx).to_owned(), Some(s.slice(idx+1, s.len()).to_owned()))
    }
}


fn handle_request(command: ~types::Command, instruments: &HashMap<~str, Instrument>) -> json::Json {

    fn get_keys(instruments: &HashMap<~str, Instrument>) -> ~[json::Json] {
				instruments.keys().map(|k| json::String(k.to_owned())).collect()
		};
 
    let (cmd, param) = *command;
		let result: json::Json;

		return match cmd.as_slice() {
				consts::GET_KEY => {
						if param.is_none() { return json::Null };
						let keys = get_keys(instruments);
		        let (first, rest) = dotsplit(param.unwrap());
        		if !instruments.contains_key(&first) { return json::Null }
            let inst = instruments.get(&first);
						inst.get_key(rest.unwrap())
				},
				consts::HAS_KEY => {
						if param.is_none() { return json::Null };
		        let (first, rest) = dotsplit(param.unwrap());
        		if !instruments.contains_key(&first) { return json::Null }
            let inst = instruments.get(&first);
						json::Boolean(inst.has_key(rest.unwrap()))
				},
				consts::GET_SUBKEYS => {
						let keys = get_keys(instruments);
						if param.is_none() { return json::List(keys) }
		        let (first, rest) = dotsplit(param.unwrap());
        		if !instruments.contains_key(&first) { return json::Null }
            let inst = instruments.get(&first);
						inst.get_subkeys(rest)
			},
				_ => json::Null
		}
}

fn instrumentation_task (receiver:Receiver<Instrument>, command_sender: Sender<types::CommandWithSender>, command_receiver: Receiver<types::CommandWithSender>){

    let config_result = get_config();
    match config_result {
      Ok(config) => { unix::init(config,  command_sender.clone());},
      Err(errstr) => fail!(errstr)
    }
    
    let mut instruments: HashMap<~str, Instrument> = HashMap::new();

    let sel = Select::new();
    let mut instrument_receiver_handle = sel.handle(&receiver);
    let mut command_handle = sel.handle(&command_receiver);
    unsafe { command_handle.add(); instrument_receiver_handle.add(); }
    
    loop {

        let ret = sel.wait();
        //add new Instrument to instruments
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
            let result = handle_request(~request, &instruments);
						cmd_response_sender.send(result);
            
        } else {fail!("select?") }
        
    }
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
        instrumentation_task(receiver, command_sender, command_receiver)
    });
    sender
}

/// register new Instrument

pub fn register(sender:Sender<Instrument>, i:Instrument) {

    sender.send(i);
}

//set up listeners
//set up exporters
