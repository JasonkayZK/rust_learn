//! Registers a mDNS service.
//!
//! Run with:
//!
//! cargo run register <service_type> <instance_name>
//!
//! Example:
//!
//! cargo run register _my-hello._udp test1
//!
//! Options:
//! "--unregister": automatically unregister after 2 seconds.

use std::{env, thread, time::Duration};

use mdns_sd::{DaemonEvent, ServiceDaemon, ServiceEvent, ServiceInfo};

fn main() {
    env_logger::init();

    // Simple command line options.
    let args: Vec<String> = env::args().collect();
    let mut should_unreg = false;
    for arg in args.iter() {
        if arg.as_str() == "--unregister" {
            should_unreg = true
        }
    }

    // Create a new mDNS daemon.
    let mdns = ServiceDaemon::new().expect("Could not create service daemon");
    let service_type = match args.get(2) {
        Some(arg) => format!("{}.local.", arg),
        None => {
            print_usage();
            return;
        }
    };
    let instance_name = match args.get(3) {
        Some(arg) => arg,
        None => {
            print_usage();
            return;
        }
    };

    // With `enable_addr_auto()`, we can give empty addrs and let the lib find them.
    // If the caller knows specific addrs to use, then assign the addrs here.
    let my_addrs = "";
    let service_hostname = format!("{}{}", instance_name, &service_type);
    let port = 3456;

    // The key string in TXT properties is case-insensitive. Only the first
    // (key, val) pair will take effect.
    let properties = [("PATH", "one"), ("Path", "two"), ("PaTh", "three")];

    // Register a service.
    let service_info = ServiceInfo::new(
        &service_type,
        instance_name,
        &service_hostname,
        my_addrs,
        port,
        &properties[..],
    )
        .expect("valid service info")
        .enable_addr_auto();

    // Optionally, we can monitor the daemon events.
    let monitor = mdns.monitor().expect("Failed to monitor the daemon");
    let service_fullname = service_info.get_fullname().to_string();
    mdns.register(service_info)
        .expect("Failed to register mDNS service");

    println!("Registered service {}.{}", &instance_name, &service_type);

    if should_unreg {
        let wait_in_secs = 2;
        println!("Sleeping {} seconds before unregister", wait_in_secs);
        thread::sleep(Duration::from_secs(wait_in_secs));

        let receiver = mdns.unregister(&service_fullname).unwrap();
        while let Ok(event) = receiver.recv() {
            println!("unregister result: {:?}", &event);
        }
    } else {
        // Query info
        thread::spawn(move || {
            // Browse for a service type.
            service_type.clone().push_str(".local.");
            let receiver = mdns.browse(&service_type).expect("Failed to browse");

            let now = std::time::Instant::now();
            while let Ok(event) = receiver.recv() {
                match event {
                    ServiceEvent::ServiceResolved(info) => {
                        println!(
                            "At {:?}: Resolved a new service: {}\n host: {}\n port: {}",
                            now.elapsed(),
                            info.get_fullname(),
                            info.get_hostname(),
                            info.get_port(),
                        );
                        for addr in info.get_addresses().iter() {
                            println!(" Address: {}", addr);
                        }
                        for prop in info.get_properties().iter() {
                            println!(" Property: {}", prop);
                        }
                    }
                    other_event => {
                        println!("At {:?} : {:?}", now.elapsed(), &other_event);
                    }
                }
            }
        });

        // Monitor the daemon events.
        while let Ok(event) = monitor.recv() {
            println!("Daemon event: {:?}", &event);
            if let DaemonEvent::Error(e) = event {
                println!("Failed: {}", e);
                break;
            }
        }
    }
}

fn print_usage() {
    println!("Usage:");
    println!("cargo run register <service_type> <instance_name> [--unregister]");
    println!("Options:");
    println!("--unregister: automatically unregister after 2 seconds");
    println!();
    println!("For example:");
    println!("cargo run register _my-hello._udp test1");
}
