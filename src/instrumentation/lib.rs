#![crate_type = "lib"]
#![crate_id = "instrumentation#0.1"]

//! Instrumentation library that will allow to expose 
//! internal metrics from your application.

#![feature(phase)]
#[phase(syntax, link)] extern crate log;

extern crate collections;
extern crate serialize;
extern crate sync;
extern crate serialize;

use std::comm::{Sender, Receiver};
use std::os::getenv;
use std::path;
use std::comm::Select;
use serialize::json;
use collections::HashMap;
use utils::dotsplit;
pub use types::{Instrument, Config, ConfigResult};
pub use consts::{CONFIG_FILE, ENV_VAR};


mod configparse;
mod unix;
mod tcp;
mod types;
mod consts;
mod utils;

pub fn get_config() -> ConfigResult {
    //! obtain configuration
    //! trying, in order:
    //! INSTRUMENTATION variable (which can contain filename or configuration itself)
    //! instrumentation.conf file, if present
    //! return empty HashMap if no configuration is present
   
    let env_var = getenv(ENV_VAR);
    if env_var.is_none() {
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


fn handle_request(command: ~types::Command,
									instruments: &HashMap<~str, Instrument>
								 ) -> json::Json {
	  //! Generate response for received command
    fn get_keys(instruments: &HashMap<~str, Instrument>) -> ~[json::Json] {
        instruments.keys().map(|k| json::String(k.to_owned())).collect()
    };
 
    let (cmd, param) = *command;
    let result: json::Json;

    let cmd_slice = cmd.as_slice();
    //WTF:changing code below to use match breaks rustc.
    result =
        if cmd_slice == consts::GET_KEY {
            if param.is_none() { return json::Null };
            let (first, rest) = dotsplit(param.unwrap());
            if rest.is_none() { return json::Null }
            if !instruments.contains_key(&first) { return json::Null }
            let inst = instruments.get(&first);
            inst.get_key(rest.unwrap())

        } else if cmd_slice == consts::HAS_KEY {

            if param.is_none() { return json::Null };
            let (first, rest) = dotsplit(param.unwrap());
            if !instruments.contains_key(&first) { return json::Null }
            let inst = instruments.get(&first);
            if rest.is_none() {return json::Null };
            json::Boolean(inst.has_key(rest.unwrap()))

        } else if cmd_slice == consts::GET_SUBKEYS {

            let keys = get_keys(instruments);
            if param.is_none() { return json::List(keys) }
            let (first, rest) = dotsplit(param.unwrap());
            if !instruments.contains_key(&first) { return json::Null }
            let inst = instruments.get(&first);
            inst.get_subkeys(rest)

        } else { json::Null };
    result
}


fn instrumentation_main(receiver:Receiver<Instrument>,
												command_sender: Sender<types::CommandWithSender>,
												command_receiver: Receiver<types::CommandWithSender>){
    //! The working horse of instrumentation.
		//! Listens for registeres instances of Instrument
		//! and generates responses for incomming commands.
    let config_result = get_config();
    match config_result {
      Ok(config) => {
          unix::init(&config.clone(),  command_sender.clone());
          tcp::init(&config.clone(),  command_sender.clone());
      },
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


pub fn init() -> Sender<Instrument>{
    //! instrumentation initialization
    //! Spawns instrumentation task
    //! and returns sender accepting Instrument instances.
    //! TODO: change result to option for
    //! better error handling, make it optionally synchronous
    let instrumentation_channel:types::InstrumentationChannel = channel();
    let (sender, receiver) = instrumentation_channel;
    let command_channel:types::CommandChannel = channel();
    let (command_sender, command_receiver) = command_channel;
    spawn(proc() {
        instrumentation_main(receiver, command_sender, command_receiver)
    });
    sender
}


pub fn register(sender:Sender<Instrument>, i:Instrument) {
    //! register Instrument
    sender.send(i);
}
