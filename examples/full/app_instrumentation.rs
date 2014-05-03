use std::comm::Sender;
use std::default::Default;
use serialize::json;

use instrumentation;

pub fn get_key(_self: &instrumentation::Instrument, key:~str) -> json::Json {

	  if key == ~"foo" {
			  json::Number(10 as f64)
		} else if key == ~"bar" {
			  json::Number(20 as f64)
		} else { json::Null }
}

pub fn get_subkeys(_self: &instrumentation::Instrument, root:Option<~str>) -> json::Json  {
	  match root {
		    None => json::List(~[json::String(~"foo"), json::String(~"bar")]),
			  Some(_) => json::Null
		}
}

pub fn init (sender:Sender<instrumentation::Instrument>){
	  let my_instrument = instrumentation::Instrument{name: "myapp", _get_key: Some(get_key), _get_subkeys: Some(get_subkeys), ..Default::default()};
		instrumentation::register(sender, my_instrument);
}
