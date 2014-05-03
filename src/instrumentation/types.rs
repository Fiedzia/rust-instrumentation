use std::comm::{Sender, Receiver};
use std::default::Default;
use collections::HashMap;
use serialize::json;


pub type GetSubkeysFunc = fn(&Instrument, Option<~str>) -> json::Json;
pub type GetKeyFunc = fn(&Instrument, ~str) -> json::Json;
pub type HasKeyFunc = fn(&Instrument, ~str) -> bool;


/// Structure representing instrument
// WTF: moving live above into struct breaks rustdoc
pub struct Instrument {

		/// Globally unique interface name
    pub name: &'static str,

		/// Optional function for obtaining list of subkeys
    pub _get_subkeys: Option<GetSubkeysFunc>,
		/// Optional function for obtaining value for given key
    pub _get_key: Option<GetKeyFunc>,
		/// Optional function for checking if given key is defined
    pub _has_key: Option<HasKeyFunc>,

}

/// Configuration
pub type Config =  HashMap<~str, ~str>;
pub type ConfigResult =  Result<Config, ~str>;


impl Instrument {
    #[allow(dead_code)]
    pub fn get_subkeys(&self, root:Option<~str>) -> json::Json {
        match self._get_subkeys {
            None => json::Null,
            Some(fn_get_subkeys) => fn_get_subkeys(self, root)
        }
    }

		#[allow(dead_code)]
    pub fn get_key(&self, key:~str) -> json::Json {
        match self._get_key {
            None => json::Null,
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
            name: "default",
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
pub type CommandResponse = json::Json;
pub type CommandResponseChannel = (Sender<CommandResponse>, Receiver<CommandResponse>);
