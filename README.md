[![Actions Status](https://github.com/0xd34b33f/taskmaster/workflows/Rust/badge.svg)](https://github.com/0xd34b33f/taskmaster/actions)

# taskmaster
Your program must be able to start jobs as child processes, and keep them alive, restarting them if necessary. It must also know at all times if these processes are alive or dead
(This must be accurate).
Information on which programs must be started, how, how many, if they must be
restarted, etc... will be contained in a configuration file, the format of which is up to you
(YAML is a good idea, for example, but use whatever you want). This configuration must
be loaded at launch, and must be reloadable, while taskmaster is running, by sending a
SIGHUP to it. When it is reloaded, your program is expected to effect all the necessary
changes to its run state (Removing programs, adding some, changing their monitoring
conditions, etc ...), but it must NOT de-spawn processes that haven’t been changed in
the reload.

Your program must have a logging system that logs events to a local file (When a
program is started, stopped, restarted, when it dies unexpectedly, when the configuration
is reloaded, etc ...)
When started, your program must remain in the foreground, and provide a control
shell to the user. It does not HAVE to be a fully-fledged shell like 42sh, but it must be
at the very least usable (Line editing, history... completion would also be nice). Take
inspiration from supervisor’s control shell, supervisorctl.

This shell must at least allow the user to: 
• See the status of all the programs described in the config file ("status" command)  
• Start / stop / restart programs  
• Reload the configuration file without stopping the main program  
• Stop the main program  
The configuration file must allow the user to specify the following, for each program  
that is to be supervised:  
• The command to use to launch the program  
• The number of processes to start and keep running  
• Whether to start this program at launch or not  
• Whether the program should be restarted always,never, or on unexpected exits
only  
• Which return codes represent an "expected" exit status  
• How long the program should be running after it’s started for it to be considered
"successfully started"  
• How many times a restart should be attempted before aborting  
• Which signal should be used to stop (i.e. exit gracefully) the program  
• How long to wait after a graceful stop before killing the program  
• Options to discard the program’s stdout/stderr or to redirect them to files  
• Environment variables to set before launching the program  
• A working directory to set before launching the program  
• An umask to set before launching the program.  
• Privilege de-escalation on launch (Needs to be started as root, so you would need
a VM for this ...)  
• Client/server archictecture to allow for two separate programs : A daemon, that
does the actual job control, and a control program, that provides a shell for the
user, and communicates with the daemon over UNIX or TCP sockets. (Very much
like supervisord and supervisorctl)
• More advanced logging/reporting facilities (Alerts via email/http/syslog/etc...)  
• Allow the user to "attach" a supervised process to its console, much in the way that
tmux or screen do, then "detach" from it and put it back in the background.
