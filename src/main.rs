use clap::{Arg, App};
use colored::*;
use std::collections::BTreeMap;
use std::net::TcpStream;

fn parse_addresses(raw: &str) -> Vec<&str> {
    let parts = raw.split(".").collect::<Vec<&str>>();

    let ips: Vec<&str> = vec![];

    let mut plages: [Vec<&str>;4] = [vec![];4];

    if parts.len() != 4 {
        return vec![];
    }

    for i in 0..4 {
        if parts[i].split("/").collect::<Vec<&str>>().len() == 2 {

            let splited = parts[i].split("/").collect::<Vec<&str>>();

            let first = match splited[0].parse::<i32>() {
                Ok(n) => n,
                Err(_e) => 22
            };

            let second = match splited[1].parse::<i32>() {
                Ok(n) => n,
                Err(_e) => 22,
            };

            for j in first..second {
                plages[i].push(&format!("{}", j));
            }

            /*
                Here is code to parse ip with format : 127.0.0.0/21

                Put after the code to generate all the ips corresponding.
            */

        } else {
            plages[i] = vec![parts[i]];
        }
    }

    vec![]
}



fn main() {

    let matches = App::new("rscan")
                    .version(env!("CARGO_PKG_VERSION"))
                    .author(env!("CARGO_PKG_AUTHORS"))
                    .about("Network scanner written in Rust")
                    .arg(Arg::with_name("ports")
                        .short("p")
                        .long("ports")
                        .value_name("PORTS")
                        .help("Sets the ports to scan")
                        .takes_value(true))
                    .arg(Arg::with_name("address")
                        .short("i")
                        .long("ip")
                        .value_name("ADDRESS")
                        .help("Sets the addresses to scan")
                        .takes_value(true)
                        .required(true))
                    .get_matches();



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