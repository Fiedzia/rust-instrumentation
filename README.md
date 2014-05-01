RMX - Rust Instrumentation tool

Simple and user-friendly instrumentation for rust applications.

General idea:

    Create interface that allows to query your rust application
    for a set of metrics, to get better overview of its state.
    For example you could use it to get amount of users connected
    to your server, its memory usage or any other value that might
    be usefull to observe. RMX is made of two parts:

    * instrumentation module that you'll need to embed in your app
    * rmx client that can connect to your app and grab requested data.

Usage:

    First, you will need cargo-lite (https://github.com/cmr/cargo-lite)
    in order to build it.
    See examples/full for code you'll need to add to you app.
    In the simplest case you'll need to add few lines of boiler plate
    to your main.rs and implement fn get_key(key:~str) -> Option(~str).

    Using attached example:

        #shell one
				cd examples/full
        cargo-lite build --force
				./hello  # now we have app running that we can query

        #shell two
        cd examples/full
        ../../rmx.py -c 'unix:///tmp/hello_instrumentation' GET_KEY myapp.bar
				$ GET_KEY myapp.bar 20

        ../../rmx.py -c 'unix:///tmp/hello_instrumentation' GET_SUBKEYS
				$ GET_SUBKEYS None ['myapp']

        ../../rmx.py -c 'unix:///tmp/hello_instrumentation' GET_SUBKEYS myapp
				$ GET_SUBKEYS myapp ['foo', ''bar]


		Note that instrumentation configuration comes from instrumentation.conf file.
    (it can be stored in INSTRUMENTATION env var as well).
    This puts application user in control if and how metrics can be accessed
    to make it as convenient for him as possible.

Configuration:

		Example instrumentation.conf:

    to enable unix socket:
		
        socket.unix on
        socket.unix.path "/tmp/hello_instrumentation"

    to enable tcp socket:
    
        socket.tcp on
        socket.tcp.port 6000
        socket.tcp.addr 127.0.0.1

Current state:

    It compiles, so I'm shipping :-)
    For now its closer to proof of concept then to usable code.
    Any error handling is missing and its likely to be buggy.
    Interface will definitely change at least a bit.


TODO:

    * Fix failure messages for closed sockets
    * Replace ~ with & whenever possible
    * Tests
    * Proper error handling
    * Include errors/metadata in returned json
    * Add http listener
    * Support for pushing metrics to zabbix/graphite/whatever
    * Write documentation
    * Pagination for GET_SUBKEYS?
    * Parse existing instrumentation.conf by rmx.
