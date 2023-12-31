
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use indicatif::{ProgressBar, ProgressStyle};

const TIMEOUT_SECS: u64 = 2;
const SERVICE_TIMEOUT_SECS: u64 = 10;
const READ_TIMEOUT_SECS: u64 = 10;

fn scan_port(ip: IpAddr, port: u16, results: Arc<Mutex<Vec<(u16, String, Option<String>, Option<String>)>>>, pb: Arc<ProgressBar>) {
    let socket_addr = SocketAddr::new(ip, port);
    let mut service_found = "Unknown".to_string();
    let mut response = None;
    let ip_string = IpAddr::from(ip).to_string();
    if let Ok(mut stream) = TcpStream::connect_timeout(&socket_addr, Duration::from_secs(TIMEOUT_SECS)) {
        let start_time = Instant::now();
        stream.set_read_timeout(Some(Duration::from_secs(READ_TIMEOUT_SECS))).unwrap();
        let mut buffer = Vec::new();
        let mut temp_buffer = [0; 1024];

        while start_time.elapsed() < Duration::from_secs(SERVICE_TIMEOUT_SECS) {
            match stream.read(&mut temp_buffer) {
                Ok(0) => break,
                Ok(bytes_read) => {
                    buffer.extend_from_slice(&temp_buffer[..bytes_read]);
                }
                Err(_) => continue,
            }
        }
        response = Some(String::from_utf8_lossy(&buffer).to_string());

        let response_lowercase = response.as_ref().unwrap().to_lowercase();
        if response_lowercase.contains("http") {
            service_found = "HTTP".to_string();
        } else if response_lowercase.contains("ssh") {
            service_found = "SSH".to_string();
        } else if response_lowercase.contains("ftp") {
            service_found = "FTP".to_string();
        } else if response_lowercase.contains("rfb") {
            service_found = "RFB".to_string();
        } else if response_lowercase.contains("smtp") {
            service_found = "SMTP".to_string();
        } else if response_lowercase.contains("plex") {
            service_found = "PLEX".to_string();
        }
    }

    let mut results = results.lock().unwrap();
    results.push((port, service_found, response, Some(ip_string)));

    pb.inc(1);
}

pub fn main_range() {
    let start_ip: Ipv4Addr = Ipv4Addr::new(192, 168, 3, 1);
    let end_ip: Ipv4Addr = Ipv4Addr::new(192, 168, 3, 10);
    let start_port = 1;
    let end_port = 65535;

    let results = Arc::new(Mutex::new(Vec::new()));

    let pb = Arc::new(ProgressBar::new((end_port - start_port + 1) as u64));
    let style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta})");

    pb.set_style(style.unwrap());

    let mut threads = vec![];

    for ip in u32::from(start_ip)..=u32::from(end_ip) {
        let ip = Ipv4Addr::from(ip).to_string();
        eprintln!("{}",ip);
        for port in start_port..=end_port {
            let ip = ip.clone();
            let results = Arc::clone(&results);
            let pb = Arc::clone(&pb);
            threads.push(thread::spawn(move || {
                if let Ok(ip_addr) = ip.parse::<IpAddr>() {
                    scan_port(ip_addr, port, results, pb);
                }
            }));
        }
        pb.reset();
    }

    for thread in threads {
        thread.join().unwrap();
    }
    
    pb.finish();

    let results = results.lock().unwrap();
    println!("Open ports:");
    for (port, service, response,ip_string) in results.iter() {
        if response.is_some() {
            println!("{:?} - {}: {} {:?}",ip_string.as_ref().unwrap(), port, service, response.as_ref().unwrap());
        }
    }
}
