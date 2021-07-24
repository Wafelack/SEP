use std::{
    io::{self, Read},
    net::TcpStream,
};

pub enum RequestType {
    Read,
}
pub struct Request {
    pub request_type: RequestType,
    pub body: Vec<u8>,
}
impl Request {
    pub fn read(mut stream: TcpStream) -> io::Result<Option<Self>> {
        let mut magic = [0; 8];
        stream.read_exact(&mut magic)?;
        if magic
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, b)| acc | (*b as u64) << ((7 - idx) * 8))
            != 3.1415926535897932f64
                .to_be_bytes()
                .iter()
                .enumerate()
                .fold(0, |acc, (idx, b)| {
                     acc | (*b as u64) << ((7 - idx) * 8)
                })
        {
            return Ok(None);
        }

        let mut raw_type = [0; 1];
        stream.read_exact(&mut raw_type)?;
        let request_type = match raw_type[0] {
            0 => RequestType::Read,
            _ => return Ok(None),
        };

        let mut raw_size = [0; 4];
        stream.read_exact(&mut raw_size)?;
        let size = raw_size
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, b)| acc | (*b as u64) << ((3 - idx) * 8));
        let mut body = vec![0; size as usize];
        stream.read_exact (&mut body)?;

        Ok(Some(Request {
            request_type,
            body,
        }))
    }
}

#[repr(u8)]
pub enum ResponseStatus {
    Success = 0,
    BadRequest = 1,
    NotFound = 2,
    ServerError = 3,
}
pub struct Response {
    pub status: ResponseStatus,
    pub body: Vec<u8>,
}
impl Response {
    pub fn serialize(self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(self.body.len() + 5);
        vec.push(self.status as u8);
        vec.extend_from_slice(&(self.body.len() as u32).to_be_bytes());
        vec.extend(self.body);
        vec
    }
}
#[cfg(test)]
mod tests {
    
    use super::{Response, ResponseStatus};

    #[test]
    fn serialize_response() {
        let mut vec = vec![0, 0, 0, 0, 14];
        vec.extend("Hello, World !".as_bytes());
        assert_eq!(&Response {
            status: ResponseStatus::Success,
            body: "Hello, World !".as_bytes().to_vec(),
        }.serialize(), &vec);
    }
}
