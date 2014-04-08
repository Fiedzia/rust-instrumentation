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
