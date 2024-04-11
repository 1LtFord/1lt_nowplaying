use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};


use crate::config;
use crate::nowplaying::NowplayingData;


pub struct WebDisplay{
    port: String,
    files:  Vec<String>,
    public: bool
}

impl WebDisplay {
    pub fn new(port: String, files: Vec<String>, public: bool) -> WebDisplay{
        let port_check: u64 = match port.parse() {
            Ok(port_check) => port_check,
            Err(_error) => panic!("config error: provided port is not a valid number")
        };
        if !(port_check >= 1024 && port_check <= 65535 || port_check == 80) {
            panic!("config error: provided port is not within accepted range (80 or 1024-65535)");
        }

        WebDisplay{port, files, public}
    }

    pub fn start(&self, rx: Arc<Mutex<mpsc::Receiver<NowplayingData>>>) {
        let listener = match TcpListener::bind("127.0.0.1:9500") {
            Ok(listener) => listener,
            Err(error) => panic!("Connont bind tcp listener to 127.0.0.1:9500 {}", error)
        };

        let mut old_data = NowplayingData{current_artist: String::new(), current_title: String::new(), current_album: String::new()};
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => old_data = WebDisplay::handle_http_request(&self, stream, &rx, old_data),
                Err(error) => panic!("Cannot handle tcp stream: {error}")
            };
        }
    }

    fn handle_http_request(&self, mut stream: TcpStream, rx: &Arc<Mutex<mpsc::Receiver<NowplayingData>>>, old_data: NowplayingData) -> NowplayingData{
        let buf_reader = BufReader::new(&mut stream);
        let request = match buf_reader.lines().next() {
            Some(request) => match request {
                Ok(request) => request,
                Err(_error) => return old_data//ignore request if faulty
            }
            None => return old_data//ignore request if faulty
        };
        let mut current_data = old_data;

        /*let (status_line, content) = match &request[..]{
            "GET /nowplaying HTTP/1.1" => {
                current_data = WebDisplay::get_nowplaying_data(rx, current_data);
                if current_data.current_album != String::new() {
                    ("HTTP/1.1 200 OK", format!("{{\"nowplaying\": {{\"title\": \"{} [{}]\", \"artist\": \"{}\"}}}}", current_data.current_title, current_data.current_album, current_data.current_artist))
                }
                else {
                    ("HTTP/1.1 200 OK", format!("{{\"nowplaying\": {{\"title\": \"{}\", \"artist\": \"{}\"}}}}", current_data.current_title, current_data.current_artist))
                }
            }
            _ => {
                let mut response = ("HTTP/1.1 403 FORBIDDEN", String::new());
                for file in &self.files {
                    if &request[..] == format!("GET {file} HTTP/1.1") {
                        match WebDisplay::get_file(file.clone()) {
                            Ok(content) => response = ("HTTP/1.1 200 OK", content),
                            Err(()) => response = ("HTTP/1.1 404 NOT FOUND", String::new())
                        }
                    }
                }
                response
            }
        };*/

        let mut status_line = "";
        let mut content: Vec<u8> = Vec::new();

        match &request[..] {
            "GET /nowplaying HTTP/1.1" => {
                current_data = WebDisplay::get_nowplaying_data(rx, current_data);
                if current_data.current_album != String::new() {
                    status_line = "HTTP/1.1 200 OK"; 
                    content = format!("{{\"nowplaying\": {{\"title\": \"{} [{}]\", \"artist\": \"{}\"}}}}", current_data.current_title, current_data.current_album, current_data.current_artist).as_bytes().to_vec();
                }
                else {
                    status_line = "HTTP/1.1 200 OK";
                    content = format!("{{\"nowplaying\": {{\"title\": \"{}\", \"artist\": \"{}\"}}}}", current_data.current_title, current_data.current_artist).as_bytes().to_vec();
                }
            }
            _ => {
                for file in &self.files {
                    if &request[..] == format!("GET {file} HTTP/1.1") {
                        match WebDisplay::get_file_binary(file.clone()) {
                            Ok(file_content) => {status_line = "HTTP/1.1 200 OK"; content = file_content;},
                            Err(()) => {status_line = "HTTP/1.1 404 NOT FOUND"; content = String::new().as_bytes().to_vec();}
                        };
                    }
                }
                if status_line == "" {
                    status_line = "HTTP/1.1 403 FORBIDDEN";
                }
            }
        }

        let length = content.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
        let mut response = response.as_bytes().to_vec();
        response.extend(content);
        match stream.write_all(response.as_slice()) {
            Ok(()) => (),
            Err(error) => panic!("error while answering http request: {error}")
        };

        current_data
    }

    fn get_nowplaying_data(rx: &Arc<Mutex<mpsc::Receiver<NowplayingData>>>, mut current_data: NowplayingData) -> NowplayingData {
        for new_data in rx.lock().unwrap().try_iter() {
            current_data = new_data
        }
        current_data
    }

    fn get_file_binary(mut filepath: String) -> Result<Vec<u8>, ()> {
        filepath = format!("{}web_display{filepath}", config::get_default_work_directory());
        println!("accessed from network: {filepath}");
        let file = match File::open(filepath) {
            Ok(file) => file,
            Err(_error) => return Err(())
        };
        let mut reader = BufReader::new(file);
        let mut content:Vec<u8> = Vec::new();
        let _ = reader.read_to_end(&mut content);
        return Ok(content)
    }
}