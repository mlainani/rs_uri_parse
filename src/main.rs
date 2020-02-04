use regex::Regex;
use std::env;

// Trying to implement RFC 7252 section 6.4.
struct CoapUri {
    host: Option<usize>,
    port: Option<u8>,
    path: Option<Vec<u8>>,
    query: Option<Vec<u8>>,
}

impl CoapUri {
    fn new() -> CoapUri {
	CoapUri {host: None, port: None, path: None, query: None}
    }
}

fn main() {
    let _uri_str1 = "coap://example.com:5683/~sensors/temp.xml";
    let _uri_str2 = "coap://EXAMPLE.com/%7Esensors/temp.xml";
    let _uri_str3 = "coap://EXAMPLE.com:/%7esensors/temp.xml";
    let _coap_uri = CoapUri {
        host: None,
        port: None,
        path: None,
        query: None,
    };

    let args: Vec<String> = env::args().collect();

    let uri_str = &args[1];
    println!("User-provided URI is {}", uri_str);

    let re = Regex::new(r"EXAMPLE").unwrap();

    match re.captures(uri_str) {
        Some(caps) => println!("Found match!: {}", &caps[0]),
        None => println!("No match found"),
    }
}
