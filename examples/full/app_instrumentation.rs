use std::comm::Sender;
use std::default::Default;
use instrumentation;

pub fn get_key(_self: &instrumentation::Instrument, key:~str) -> Option<~str> {

		println!("here::{}", key);
	  if key == ~"foo" {
			  Some(~"10")
		} else if key == ~"bar" {
			  Some(~"20")
		} else { None }
}

pub fn get_subkeys(_self: &instrumentation::Instrument, root:Option<~str>) -> Option<~[~str]>  {
	  match root {
		    None => Some(~[~"foo", ~"bar"]),
			  Some(_) => None
		}
}

pub fn init (sender:Sender<instrumentation::Instrument>){
	  let mut my_instrument = instrumentation::Instrument{name: "myapp", _get_key: Some(get_key), _get_subkeys: Some(get_subkeys), ..Default::default()};
		instrumentation::register(sender, my_instrument);
}
