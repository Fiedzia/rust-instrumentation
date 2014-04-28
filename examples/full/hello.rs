#![crate_id = "hello#1.0"]
extern crate serialize;

extern crate instrumentation;

use std::io::timer::sleep;

mod app_instrumentation;


fn main(){
	  let mut foo:int  = 0;
		let chan = instrumentation::init();
	  app_instrumentation::init(chan);
	  println!("starting hello");
		//lets do something to keep it running
		loop {
			println!("I'm working");
			sleep(2*1000);
		}
}
