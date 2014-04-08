use std::comm::{Sender, Receiver};

//#[cfg(instrumentation)]
pub struct Instrument {
    pub name: &'static str
}

pub trait Instrumentable {

    fn get_subkeys(root:Option<~str>) -> Option<~[~str]> { None }

		fn get_key(key:~str) -> Option<~str> { None }

		fn has_key(key:~str) -> bool { false }

}


fn start (r:Receiver<Instrument>){

    //parse config
		//set up listener tasks
    loop {
		    let i:Option<Instrument> = r.recv_opt();
				if !i.is_some() { break; }
        let response = match i {
					Some(x) => x.get_subkeys(None),
					None => ~[]
				};
				println!("{}", response);
		}
}


pub type IChan = (Sender<Instrument>, Receiver<Instrument>);

//init
pub fn init() -> Sender<Instrument>{
    let ic:IChan = channel();
    let (s, r) = ic;
    spawn(proc() {
			  start(r)
		});
		s
}

//register

pub fn register(s:Sender<Instrument>, i:Instrument) {

		s.send(i);
}

//set up listeners
//set up exporters
