# daytimeserver
Rust equivalent of the Professor George Porter CSE 124 daytimeserver.go demo

Creates a new tcp listener on localhost at the specified port and waits for a connection. Once connected it will send the current time and close out the connection

Usage: ./daytimeserver port
