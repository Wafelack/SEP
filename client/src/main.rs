use sep::{Request, RequestType, Response, ResponseStatus};
use std::{env, io::{self, Write}, net::TcpStream};

fn main() -> io::Result<()> {
    let link = match env::args().nth(1) {
        Some(s) => s,
        None => return Ok(()),
    };
    let (link, path) = match link.split_once(':') {
        Some(x) => x,
        None => return Ok(()),
    };
    let mut stream = TcpStream::connect(&format!("{}:31415", link))?;
    stream.write_all(&Request {
        request_type: RequestType::Read,
        body: path.as_bytes().to_vec(),
    }.serialize())?;
    let response = Response::read(&mut stream)?;
    println!("{:?}", response);
    Ok(())
}
