// Trying to implement RFC 7252 section 6.4. 
struct Uri {
    host: Option<usize>,
    port: Option<u8>,
    path: Option<Vec<u8>>,
    query: Option<Vec<u8>>
}

fn main() {
    let s1 = "coap://example.com:5683/~sensors/temp.xml";
    let s2 = "coap://EXAMPLE.com/%7Esensors/temp.xml";
    let s3 = "coap://EXAMPLE.com:/%7esensors/temp.xml";

    println!("Hello, world!");
}
