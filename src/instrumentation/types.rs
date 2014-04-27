use std::comm::{Sender, Receiver};
use std::default::Default;
use std::os::getenv;
use std::path;
use collections::HashMap;
use std::result::Result;



pub type GetSubkeysFunc = fn(&Instrument, Option<~str>) -> Option<~[~str]>;
pub type GetKeyFunc = fn(&Instrument, ~str) -> Option<~str>;
pub type HasKeyFunc = fn(&Instrument, ~str) -> bool;


pub struct Instrument {
    
    pub name: &'static str,
    pub _get_subkeys: Option<GetSubkeysFunc>,
    pub _get_key: Option<GetKeyFunc>,
    pub _has_key: Option<HasKeyFunc>,

}

//#[cfg(instrumentation)]


impl Instrument {
    #[allow(dead_code)]
    pub fn get_subkeys(&self, root:Option<~str>) -> Option<~[~str]> {
        match self._get_subkeys {
            None => None,
            Some(fn_get_subkeys) => fn_get_subkeys(self, root)
        }
    }

		#[allow(dead_code)]
    pub fn get_key(&self, key:~str) -> Option<~str> {
        match self._get_key {
            None => None,
            Some(fn_get_key) => fn_get_key(self, key)
        }
    }

    #[allow(dead_code)]
    pub fn has_key(&self, key:~str) -> bool {
        match self._has_key {
            None => false,
            Some(fn_has_key) => fn_has_key(self, key)
        }
    }
}

impl Default for Instrument {
    fn default () -> Instrument {
        Instrument {
            name: "",
            _get_subkeys:  None,
            _get_key: None,
            _has_key: None,
        }
    }
}




pub type InstrumentationChannel = (Sender<Instrument>, Receiver<Instrument>);
pub type Command = (~str, Option<~str>);
pub type CommandWithSender = (Sender<CommandResponse>, Command);

pub type CommandChannel = (Sender<CommandWithSender>, Receiver<CommandWithSender>);
pub type CommandResponse = Option<~str>;

//pub type CommandChannel = (Sender<Command>, Receiver<Command>);

pub type CommandResponseChannel = (Sender<CommandResponse>, Receiver<CommandResponse>);


