use clap::{Arg, App};
use colored::*;
use std::collections::BTreeMap;
use std::net::TcpStream;
use std::time::Instant;

fn parse_addresses(raw: &str) -> Vec<String> {
    let parts = raw.split(".").map(|s| { s.to_string() }).collect::<Vec<String>>();

    let mut ips: Vec<String> = vec![];

    let mut plages: [Vec<String>;4] = [vec![], vec![], vec![], vec![]];

    if parts.len() != 4 {
        return vec![];
    }

    for i in 0..4 {
        if parts[i].split("/").map(|s| { s.to_string() }).collect::<Vec<String>>().len() == 2 {

            let splited = parts[i].split("/").collect::<Vec<&str>>();

            let first = match splited[0].parse::<u32>() {
                Ok(n) => n,
                Err(_e) => 8
            };

            let second = match splited[1].parse::<u32>() {
                Ok(n) => n,
                Err(_e) => 9,
            };

            for j in first..second {
                plages[i].push(format!("{}", j));
            }

            /*
                Here is code to parse ip with format : 127.0.0.0/21

                Put after the code to generate all the ips corresponding.
            */
        } else {
            plages[i] = vec![parts[i].clone()];
        }
    }

    for first in &plages[0] {
        for second in &plages[1] {
            for third in &plages[2] {
                for fourth in &plages[3] {
                    ips.push(format!("{}.{}.{}.{}", first, second, third, fourth));
                }
            }
        }
    }

    ips
}

fn parse_ports(raw: &str) -> Vec<String> {
    let parts = raw.split(",").map(|s| { s.to_string() }).collect::<Vec<String>>();
    let mut ports: Vec<String> = vec![];
    if parts.len() < 1 {
        return vec![];
    }
    
    for part in parts {
        if part.split("/").map(|s| { s.to_string() }).collect::<Vec<String>>().len() == 2 {
            let splited = part.split("/").collect::<Vec<&str>>();

            let first = match splited[0].parse::<u32>() {
                Ok(n) => n,
                Err(_e) => 22
            };

            let second = match splited[1].parse::<u32>() {
                Ok(n) => n,
                Err(_e) => 23,
            };

            for i in first..second {
                ports.push(format!("{}", i));
            }
        } else {
            ports.push(part);
        }
    }

    ports

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_rparser() {
        assert_eq!(parse_addresses("127.0.0/2.1/3"), vec!["127.0.0.1", "127.0.0.2", "127.0.1.1", "127.0.1.2"]);
    }

    #[test]
    fn ports_parser() {
        assert_eq!(parse_ports("55,5/8"), vec!["55", "5", "6", "7"]);
    }

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
                        .takes_value(true)
                        .required(true))
                    .arg(Arg::with_name("addresses")
                        .short("i")
                        .long("ips")
                        .value_name("ADDRESS")
                        .help("Sets the addresses to scan")
                        .takes_value(true)
                        .required(true))
                    .get_matches();

        let ips = parse_addresses(matches.value_of("addresses").unwrap());
        let ports = parse_ports(matches.value_of("ports").unwrap());

        let mut total = 0usize;

        let start = Instant::now();

        for ip in ips {

            let mut status: BTreeMap<&String, colored::ColoredString> = BTreeMap::new();

            let mut open_amount = 0usize;
            let mut closed = 0usize;

            for port in &ports {
                let open = match TcpStream::connect(&format!("{}:{}", ip, port)) {
                    Ok(_) => true,
                    Err(_e) => false,
                };

                if open {
                    status.insert(port, "Open".green());
                    open_amount+=1;
                } else {
                    status.insert(port, "Closed".red());
                    closed+=1;
                }
            }

            

            println!("Target address : {}\n", &ip);
            total+=1;

            for (port, stat) in status.iter() {
                println!("{}.....{}", port, stat);
            }
            println!("\nTotal : {} ports scanned : {} open, {} closed.", open_amount + closed, open_amount, closed);
            println!("\n======================================================\n");
        }
        let elapsed = start.elapsed();
        println!("\n{} address scanned in {:.2?}", total, elapsed);
    }
