use std::net::{TcpStream};
use std::io::{BufReader, BufWriter, BufRead, Write};
use std::collections::{HashMap};

struct HttpRequest {
    method: String,
    resource: String, 
    version: String,
    headers: HashMap<String, String>,
    body: String
}

impl HttpRequest {
    fn new() -> HttpRequest {
        HttpRequest { 
            method: String::new(),
            resource: String::new(),
            version: String::new(),
            headers: HashMap::new(),
            body: String::new()
        }
    }

    fn get<S: Into<String>>(&mut self, resource: S) -> &mut HttpRequest {
        self.method = String::from("GET");
        self.resource = resource.into();
        self
    }

    fn post<S: Into<String>>(&mut self, resource: S) -> &mut HttpRequest {
        self.method = String::from("POST");
        self.resource = resource.into();
        self
    }

    fn head<S: Into<String>>(&mut self, resource: S) -> &mut HttpRequest {
        self.method = String::from("HEAD");
        self.resource = resource.into();
        self
    }

    fn put<S: Into<String>>(&mut self, resource: S) -> &mut HttpRequest {
        self.method = String::from("PUT");
        self.resource = resource.into();
        self
    }

    fn delete<S: Into<String>>(&mut self, resource: S) -> &mut HttpRequest {
        self.method = String::from("DELETE");
        self.resource = resource.into();
        self
    }

    fn version<S: Into<String>>(&mut self, version: S) -> &mut HttpRequest {
        self.version = version.into();
        self
    }

    fn header<S: Into<String>>(&mut self, name: S, value: S) -> &mut HttpRequest {
        self.headers.insert(name.into(), value.into());
        self
    }

    fn body<S: Into<String>>(&mut self, body: S) -> &mut HttpRequest {
        self.body = body.into();
        self
    }

    fn to_string(&self) -> Option<String> {
        let mut ret = String::new();
        ret += format!("{} {} {}\r\n", self.method.as_str(), self.resource.as_str(), self.version.as_str()).as_str();
        for (name, value) in &self.headers {
            ret += format!("{}: {}\r\n", name.as_str(), value.as_str()).as_str();
        }
        ret.push_str(self.body.as_str());
        ret.push_str("\r\n\r\n");
        Some(ret)
    }
}

impl FromStr for HttpRequest {
    fn from_str (s: &str) -> Result<Self, Self::Err> {
        enum State {
            METHOD,
            RESOURCE,
            VERSION,
            HEADER_NAME,
            HEADER_VALUE,
            BODY,
            DONE
        }
        let mut state = State::METHOD;
        let mut httpreq = HttpRequest::new();

        loop {
            match state {
                METHOD => {},
                RESOURCE => {},
                VERSION => {},
                HEADER_NAME => {},
                HEADER_VALUE => {},
                BODY => {},
                DONE => break
            }
        }

        Ok(httpreq)
    }
}

fn main() {
    //let endpoint = "kr.ncsoft.com:80";
    let endpoint = "127.0.0.1:8000";

    let stream = match TcpStream::connect(endpoint) {
        Ok(stream) => stream,
        Err(err) => panic!("connect fail : {:?}", err)
    };

    println!("connected.");

    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    write!(writer, "{}", 
        HttpRequest::new().get("/").version("HTTP/1.1")
            .header("Host", "kr.ncsoft.com")
            .header("Content-Length", "0")
            .to_string().unwrap()
    );

    match writer.flush() {
        Ok(_) => {},
        Err(err) => panic!("writer flush fail : {:?}", err)
    }

    println!("request sent.");

    let mut response = String::new();
    loop {
        match reader.read_line(&mut response) {
            Ok(len) => {
                if len == 0 {
                    println!("eof.");
                    break;
                }
            },
            Err(err) => {
                panic!("read line fail : {:?}", err)
            }
        }

        if response == "\r\n\r\n" {
            break;
        }

        println!("{}", response.trim());
        response.clear();
    }
}
