# Sample Application

# installation instructions

YOu will need to have npm and cargo already installed.

Inside the frontend directory; run `npm install`

# debugging instructions

On the frontend, you can right click "inspect element" to see errors and what the HTML is like.

On the backend, running `info!()` or `error!()` will print out to the system log.

On linux, you can access this with:

``` sh
$ tail -f /var/log/syslog
```
On mac, you can access this with:

``` sh
$ tail -f /var/log/system.log
```

Note that in both cases, not all the messages are produced by SunswiftOS (i.e. some messages
are from other apps running on your system). 
