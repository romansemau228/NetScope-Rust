use std::{io};
use std::net::{Ipv4Addr, ToSocketAddrs};
use std::net::TcpStream;
use std::time::{Duration, Instant};
use std::net::SocketAddr;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
fn main () {
    let mut addresses = vec![
    String::from("192.168.1.1"),
    String::from("google.com"),
    String::from("8.8.8.8"),
    String::from("256.256.256.256"),
    String::from("1.1.1.1"),
];
    loop {
        
        println!("================================");
        println!("         NetScope v0.2");
        println!("================================");
        println!("1. Connection information");
        println!("2. Internet connectivity test");
        println!("3. DNS lookup");
        println!("4. Test IPv4s");
        println!("5. Add IPv4");
        println!("6. Remove IPv4");
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
            4 => test_ipv4s(&addresses),
            5 => add_ipv4(&mut addresses),
            6 => remove_ipv4(&mut addresses),
            0 => {
                println!("Goodbye!");
                break;
                }
            _ => println!("Unknown option!"),
        }
    }
}

fn connection_info() {
    let interfaces = match NetworkInterface::show() {
        Ok(interfaces) => interfaces,
        Err(error) => {
            println!("Failed to get interfaces: {error}");
            return;
        }
    };
    let mut counter = 0;
    for interface in interfaces {
        for addr in interface.addr {
        counter += 1;
        let ip_type: &str = if addr.ip().is_ipv4() {
            "IPv4"
        }
        else {
            "IPv6"
        };
        println!("{counter}. Interface: {}", interface.name);
        println!("IP: {}", addr.ip());
        println!("Type: {}", ip_type);
        println!("---");
    }
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
fn find_first_ipv4(addresses: &[String]) -> Option<String> {
    for addr in addresses {
        if addr.parse::<Ipv4Addr>().is_ok() {
            return Some(addr.to_string());
        }
    }
    None
}
fn find_all_ipv4(addresses: &[String]) -> Vec<String> {
    let mut ipv4_addresses: Vec<String> = vec![];
    for addr in addresses {
        if addr.parse::<Ipv4Addr>().is_ok() {
        ipv4_addresses.push(addr.to_string());
        }
    };
    ipv4_addresses
}

fn test_ipv4s(addresses: &[String]) {
    let first_ipv4 = find_first_ipv4(addresses);
    match first_ipv4 {
        Some(ip) => println!("First IPv4: {}", ip),
        None => println!("No IPv4 addresses found"),

    }
    let ipv4s = find_all_ipv4(addresses);
    println!("All IPv4 addresses: {ipv4s:#?}");
    println!("Total IPv4 addresses: {}", ipv4s.len());
}
fn add_ipv4(addresses: &mut Vec<String>) {
    println!("Enter IPv4_address.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();
    if input.parse::<Ipv4Addr>().is_ok() {
        if addresses.contains(&input.to_string()) {
            println!("This address already exists!");
            return;
        }
        else {
        addresses.push(input.to_string());
        }
    }
    else {
        println!("Invalid IPv4 address!")
    }
}
fn remove_ipv4(addresses: &mut Vec<String>) {
    println!("Enter IPv4_address.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();
        if input.parse::<Ipv4Addr>().is_err() {
        println!("Invalid IPv4");
        }
        else {
        let index = addresses.iter().position(|addr| addr == input);
        match index {
            Some(index) => {
                let removed = addresses.remove(index);
                println!("Removed: {removed}");
            }
            None => {
                println!("IPv4 address not found!");
            }
        }
        }
    }