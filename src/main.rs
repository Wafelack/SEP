use colored::*;
use std::collections::BTreeMap;
use std::net::TcpStream;

fn help() {
    eprintln!("Usage : rscan $IP $FIRST_PORT-$LAST_PORT");
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        help();
        std::process::exit(-1);   
    }
    let ip = &args[1].as_str();
    let (start, end): (i32, i32) = {
        let ports = &args[2].split('-').collect::<Vec<&str>>();

        if ports.len() != 2 {
            eprintln!("Invalid ports");
            help();      
            std::process::exit(-1);   
        }
        let start = match ports[0].parse::<i32>() {
            Ok(i) => i,
            Err(_e) => {
                eprintln!("Ports have to be valid integers");
                help();
                std::process::exit(-1);   
            }
        };
        let end = match ports[1].parse::<i32>() {
            Ok(i) => i,
            Err(_e) => {
                eprintln!("Ports have to be valid integers");
                help();
                std::process::exit(-1);   
            }
        };
        (start, end + 1)
    };

    let mut ports = BTreeMap::new();
    let mut open = 0usize;
    let mut closed = 0usize;

    let center = end / 2;



    for port in start..end {
        if let Ok(_stream) = TcpStream::connect(format!("{}:{}", ip, port)) {
            ports.insert(port, "Open".green());
            open+=1;
        } else {
            ports.insert(port, "Closed".red());
            closed+=1;
        }  
    }

    println!("Target : {}\n", ip);
    for (port, status) in ports.iter() {
        println!("Port {}.....{}", port, status);
    }
    println!("\nTotal : {} ports : {} open, {} closed", open + closed, open, closed);
}
