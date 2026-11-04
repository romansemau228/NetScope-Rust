use std::{io};
use std::net::ToSocketAddrs;
use get_if_addrs::get_if_addrs;
use std::net::TcpStream;
use std::time::{Duration, Instant};
use std::net::SocketAddr;
fn main () {
    
    loop {
        
        println!("================================");
        println!("         NetScope v0.1");
        println!("================================");
        println!("1. Connection information");
        println!("2. Internet connectivity test");
        println!("3. DNS lookup");
        println!("0. Exit");
        println!("Select option: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice: u32 = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        match choice {
            1 => connection_info(),
            2 => connectivity_test(),
            3 => dns_lookup(),
            0 => {
                println!("Goodbye!");
                break;
                }
            _ => println!("Unknown option!"),
        }
    }
}

fn connection_info() {
    let interfaces = match get_if_addrs() {
        Ok(interfaces) => interfaces,
        Err(error) => {
            println!("Failed to get interfaces: {error}");
            return;
        }
    };
    let mut counter = 0;
    for interface in interfaces {
        counter += 1;
        let ip_type: &str = if interface.ip().is_ipv4() {
            "IPv4"
        }
        else {
            "IPv6"
        };
        println!("{counter}. Interface: {}", interface.name);
        println!("IP: {}", interface.ip());
        println!("Type: {}", ip_type);
    }
}
fn connectivity_test() {
    println!("Running connectivity test...");
    let address: SocketAddr = "1.1.1.1:443".parse().unwrap();
    let start = Instant::now();
    let result = TcpStream::connect_timeout(&address, Duration::from_secs(5));
    match result {
        Ok(_) => {
            println!("Connected!")
        }
        Err(error) => {
            println!("Connection failed: {error}");
        }
    }
    let elapsed = start.elapsed();
    println!("Latency: {:?}", elapsed);
}
fn dns_lookup() {
    let mut domain = String::new();

    println!("Enter domain:");
    io::stdin().read_line(&mut domain).expect("Input error");
    let domain = domain.trim();
    if domain.is_empty() {
        println!("Domain cannot be empty!");
        return;
    }

    let result = (domain, 80).to_socket_addrs();
        match result {
        Ok(addresses) => {
            let mut counter = 0;

            for address in addresses {
                counter += 1;
                let ip_type = if address.is_ipv4() {
                    "IPv4"
                }
                else {
                    "IPv6"
                };
                println!("{counter}. {} [{ip_type}]", address.ip());  
                
            }
            if counter == 0 {
            println!("No addresses found!"); 
            }
        }
        Err(error) => {
            println!("DNS lookup failed: {}", error)
        }
    }
}