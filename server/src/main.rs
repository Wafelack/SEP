use sep::*;
use std::{
    fs,
    io::{self, Write},
    net::TcpListener,
    path::Path,
};

const ADDRESS: &str = "0.0.0.0:31415";

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(ADDRESS)?;
    println!("[+] Listener bound to {}...", ADDRESS);

    for stream in listener.incoming() {
        let mut stream = stream?;
        let request = match Request::read(&mut stream)? {
            Some(r) => r,
            None => {
                stream.write(
                    &Response {
                        status: ResponseStatus::BadRequest,
                        body: vec![],
                    }
                    .serialize(),
                )?;
                continue;
            }
        };
        println!("[+] Received request: {:?}", request);

        let response = match request.request_type {
            RequestType::Read => {
                let raw_link = String::from_utf8_lossy(&request.body).to_string();
                println!("[+] Received read request for path `{}`.", raw_link);
                let depth = raw_link
                    .split('/')
                    .filter(|s| !s.is_empty())
                    .map(|e| if e == ".." { -1 } else { 1 })
                    .fold(0, |acc, v| acc + v);
                if depth < 0 {
                    Response {
                        status: ResponseStatus::NotFound,
                        body: vec![],
                    }
                } else {
                    let secured_link = format!("./{}", raw_link);
                    let mut link = Path::new(&secured_link);
                    let index = format!("{}/index.sef", secured_link);
                    if link.is_dir() {
                        link = Path::new(&index);
                    }
                    if !link.exists() {
                        Response {
                            status: ResponseStatus::NotFound,
                            body: vec![],
                        }
                    } else {
                        Response {
                            status: ResponseStatus::Success,
                            body: fs::read(link)?,
                        }
                    }
                }
            }
        };
        println!("[+] Replied with {:?}", response);
        stream.write_all(&response.serialize())?;
    }
    Ok(())
}
