use std::comm::Sender;
use instrumentation;

pub type MyInstrumentation = instrumentation::Instrument;


impl instrumentation::Instrumentable for MyInstrumentation {

		fn get_key(key:~str) -> Option<~str> {

			  if key == ~"foo" {
					  Some(~"10")
				} else if key == ~"bar" {
					  Some(~"20")
				} else { None }
		}

    fn get_subkeys(root:Option<~str>) -> Option<~[~str]>  {
			  match root {
				    None => Some(~[~"foo", ~"bar"]),
					  _ => None
				}
		}


}

pub fn init (s:Sender<instrumentation::Instrument>){
	  let mi:MyInstrumentation = instrumentation::Instrument {name: "myapp"};
		instrumentation::register(s, mi);
}
