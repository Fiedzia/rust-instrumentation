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

    See examples/full for code you'll need to add to you app.
    In the simplest case you'll need to add few lines of boiler plate
    to your main.rs and implement fn get_key(key:~str) -> Option(~str).

    Runtime usage:

        INSTRUMENTATION="socket.unix on; socket.unix.file /tmp/instr"; ./your_app
        rmx.py -c '/tmp/instr' GET_KEY yourapp.foo

        $ 7

		Note that instrumentation configuration comes from env variable
    (it can be stored in a file as well). This allows user/system admin
    to control if and how metrics can be accessed to make it as convenient
		as possible.


TODO:

    * Code cleanup
    * Tests
    * Proper error handling
    * Use rust logging instead of println
    * Add tcp/http/websockets listeners
    * support for pushing metrics to zabbix/graphite/whatever
    * Write documentation
    * Pagination for GET_SUBKEYS?
