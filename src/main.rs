// use clap::Parser;  

// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Cli {

//     ethernet: Option<u8>, 
//     arp: Option<u8>,
//     tcp: Option<u8>,
//     interface: Option<String>, 
// }
extern crate pnet;

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet, MutablePacket};
use pnet::packet::tcp::TcpPacket;
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};

fn print_all_interface(interfaces : &Vec<NetworkInterface>){ 

    //print all the interfaces 
    for (i, intf) in interfaces.iter().enumerate() { 

        println!("{}.{}", i, intf.name); 
    }
}

fn main() {

    // let cli = Cli::parse();
    
    // if let Some(_) = cli.ethernet { 
    //     println!("Ethernet flag is ON"); 
    // }
    // if let Some(_) = cli.arp { 
    //     println!("ARP flag is ON"); 
    // }
    // if let Some(_) = cli.tcp { 
    //     println!("TCP flag is ON"); 
    // }

    // match cli.interface { 
    //     Some(interface) => println!( "Iterface is { }", interface),
    //     None => eprintln!("No interface provided")
    // }; 

    let interfaces = datalink::interfaces();
    
    /* print all interfaces */
    print_all_interface(&interfaces); 

    let interface_name : String = String::from("eno1"); 

    // closure function to get a specific function 
    let interface_names_match =
    |iface: &NetworkInterface| iface.name == interface_name;

    let interface = interfaces.into_iter()
    .filter(interface_names_match)
    .next()
    .unwrap();

    
    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
    };

    loop { 

        match rx.next() { 
            Ok(packet) => { 
                let packet = TcpPacket::new(packet); 
                match packet { 
                    Some(tp) => { 
                        println!("got tcp packet : "); 
                        println!(" source address {}\n
                                   destination address {}\n
                                   ", tp.get_source(), tp.get_destination()); 
                    }, 
                    None => (), 
                }

            }, 
            Err(e) => { 
                panic!("An error occurred while reading: {}", e);
            }

        }
    }
    
}   
