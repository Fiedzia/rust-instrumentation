//#![feature(macro_rules)]
//#![feature(phase)]


mod instrumentation;
mod app_instrumentation;



fn main(){
	  let mut foo:int  = 0;
		let chan = instrumentation::init();
	  app_instrumentation::init(chan);
	  println!("starting hello");
		loop {
			foo += 1;
			if foo > 100_000 {
				foo = 0;
			}
		}
}
