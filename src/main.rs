use std::net::TcpListener;
use std::env::args;
use std::io::{stdin, Write,Read};

// Implementation of RFC 865: Quote of the Day
// https://tools.ietf.org/html/rfc865
fn main() {
	// Read quote from stdin
	let mut quote: Vec<u8> = Vec::new();
	let _ = stdin().read_to_end(&mut quote).unwrap();

	// Use port passed as argument, or 17
	let port = args().nth(1).map_or(17, |x| x.parse().unwrap());

	// Attempt to bind address
	let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();

	// Broadcast quotes
	for connection in listener.incoming() {
		let _ = connection.and_then(|mut stream| stream.write(&quote[..]));
	}
}
