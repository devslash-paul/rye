//rye.rust
use std::net::TcpStream;

pub struct IRCClient{
    baseurl: String,
    port: i32,
    identifier: String,
    stream: Option<TcpStream>,
}

impl IRCClient {

    ///```
    ///# use rye::IRCClient;
    ///IRCClient::new("hi");
    ///```
    pub fn new(base: &str) -> IRCClient {
        let result = IRCClient{ baseurl: base.to_string(), port: 10, ..Default::default() };
        result        
    }

    ///```
    ///# use rye::IRCClient;
    ///let test = IRCClient::new("www.google.com");
    ///assert_eq!( "baseurl: www.google.com", test.str() );
    pub fn str(&self) -> String {
        format!("baseurl: {}", self.baseurl)
    }

    pub fn connect(&mut self) {
        let stream = TcpStream::connect("http://www.google.com");

        match stream {
            Ok(s) => self.stream = Some(s),
            Err(_) => panic!("Problem creating stream")
        }
    }
}

impl Default for IRCClient {
    fn default() -> IRCClient {
        IRCClient { baseurl:"".to_string(), port:8080, identifier:"abc".to_string(), stream:None }
    }
}


#[test]
#[should_panic]
fn can_create_stream() {
    let mut f = IRCClient::new("http://www.google.com");
    f.connect();
}

#[test]
#[should_panic]
fn can_create_impl() {
    let mut f:IRCClient = IRCClient::new("swog");

    f.connect();

    assert!(f.baseurl == "swog");
    assert!(f.port == 10);
}

#[test]
fn can_create_default() {
    let f = IRCClient{ baseurl:"Abc".to_string(), ..Default::default() };
}

