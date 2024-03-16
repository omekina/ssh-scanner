mod ssh;
mod ip;
use std::{time::Duration, sync::{Arc, Mutex}, cell::RefCell};
use regex::Regex;
use tokio::sync::Semaphore;


const ASYNC_PROCESSES: usize = 1024;
/**
* In milliseconds
*/
const TIMEOUT_AFTER: u64 = 1000;


#[tokio::main]
async fn main() {
    let arguments: Vec<String> = std::env::args().take(2).collect();
    if arguments.len() != 2 {
        println!("No IP range was specified");
        return;
    }
    let expression = Regex::new(r"(?<ip>([0-9]{1,3}\.){3,3}[0-9]{1,3})/(?<mask>[0-9]+)").unwrap();
    let Some(caps) = expression.captures(&arguments[1]) else {
        println!("No valid V4 argument was specified");
        return;
    };

    let mask: u8 = (&caps["mask"]).parse().unwrap();
    let (subnet_start, subnet_end) = ip::get_subnet_bounds(ip::parse(&caps["ip"]), mask);

    if subnet_start == subnet_end {
        println!("Scanning {}", ip::build(subnet_start));
    } else {
        println!("Scanning {} - {}", ip::build(subnet_start), ip::build(subnet_end));
    }

    let semaphore = Arc::new(Semaphore::new(ASYNC_PROCESSES));
    let total_found: Arc<Mutex<RefCell<u32>>> = Arc::new(Mutex::new(RefCell::new(0)));
    
    for ip_binary in subnet_start..=subnet_end {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let current_total = total_found.clone();
        tokio::spawn(async move {
            let current_ip: String = ip::build(ip_binary);
            match tokio::time::timeout(Duration::from_millis(TIMEOUT_AFTER), ssh::get_banner(&current_ip)).await {
                Ok(Some(banner)) => {
                    println!("{} -> {}", current_ip, banner);
                    *current_total.lock().unwrap().borrow_mut() += 1;
                },
                _ => {},
            }
            drop(permit);
        });
    }

    let _ = semaphore.acquire_many_owned(ASYNC_PROCESSES as u32).await.unwrap();
    println!("Scan done - found {}", total_found.lock().unwrap().borrow());
}
