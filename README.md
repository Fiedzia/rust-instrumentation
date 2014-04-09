Rust Instrumentation

Simple and user-friendly instrumentation for rust applications.

Usage:

    Application code:

        your_app/app_instrumentation.rs:

            
            use std::comm::Sender;
            use std::default::Default;
            use instrumentation;
            
            pub fn get_key(_self: &instrumentation::Instrument, key:~str) -> Option<~str> {
            
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


        main.rs:

            use instrumentation;

            fn main(){
                //...
                let chan = instrumentation::init();
                app_instrumentation::init(chan);
            }


    Runtime usage:

        INSTRUMENTATION="socket.unix on; socket.unix.file /tmp/instr"; ./your_app
        rmx -p socket.unix -l /tmp/instr myapp.foo
        


Ideally:

 - macros for various types of metrics (time measurement, counter, event log, histogram, time histogram
   (maybe there should only be events?)
 - developer defines possible metrics
 - user defines simple file defining which crates/metrics should be gathered/exported
   how should they be processed and how to store/expose them
 

What rust provides:

  - #cfg macro providing optional inclusion of object
  - cfg option can be passed as an argument to rustc
  - include! macro

Instrumentation macro ideas:


1) by passing data to macro system
event!("foo.x")
increment!("foo.y", val)
set!("foo.databases."+dbname+".size",  dbsize)


Pros:

 - easy to use
 - minimal amount of code written

Cons:

 - lots of data being passed
 - have to be applied in every place that modifies instrumented data


2) by defining function used to gather data

in your crate:

 - add file named instrumentation 

 - define function for every key
 - define keys, or function that

Pros:

 - more verbose
 - 


3) go way (same as 2), but functions return json)


 - define functions returning json

Pros:

 - works for go
 - flexibility of json
 - 

Cons:

 - still more verbose then macro
 - 

4) Dtrace/systemtap-like

  Hooks for user-defined scripts that can inspect and extract
  whatever user may want to get from data.



config ideas:

#expose data
http on
http.port 2000
http.listen 0.0.0.0
http.ui on #include built-in user interface

socket.unix on
socket.unix.path /tmp/whatever

socket.tcp on
socket.tcp.port 4000


websocket on
...

dbus.on
...

#push

output f1 file "/tmp/f1" [append=true]

push foo.x to f1 every 1min [as foo.x.1min]


output g1 graphite 
push foo.x to g1 every 1s

output h1 http "http://example.com/userdata"
push foo.x to h1 every 10min
