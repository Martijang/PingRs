use clap::Parser;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

fn send_request(address: &String, option_port: Option<i32>, ttl: Option<u32>) 
-> Result<Duration, Box<dyn std::error::Error>>
{
    let now = std::time::Instant::now();

    let port = match option_port{ Some(val) => val, None => 80 };
    let url = format!("{}:{}", address, port);

    let connect = TcpStream::connect(&url)?;

    if let Some(val) = ttl{
        connect.set_ttl(val)?;
    }else{
        connect.set_ttl(117)?;
    }

    let resolved_addr:Vec<_> = url.to_socket_addrs()?.collect();
    let duration = now.elapsed();

    println!("\tResponse of {address} address:{resolved_addr:?}, time={duration:?} TTL={:?}", 
    connect.ttl()?
    );
    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok(duration)
}

fn get_average(args: &String, port: Option<i32>, ttl: Option<u32>, echo: Option<u32>) 
-> Result<Duration, Box<dyn std::error::Error>>
{

    let mut count = match echo {Some(val) => val, None => 5};
    let mut average: Duration = Duration::ZERO;
        loop {
            if count == 0 {
                break;
            }
            let value = send_request(&args, port, ttl)?;
            average += value;
            count -= 1;
        }
    Ok(average / 5)
}

/// Ping tool written in rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// target address 
    address: String,

    ///Target port (default: 80)
    #[arg(short, long)]
    port: Option<i32>,

    ///Amount of TTL(Time To Live). Default is 117
    #[arg(short, long)]
    ttl: Option<u32>,

    ///Amount of request to send 
    #[arg(short, long)]
    count: Option<u32>
}

fn main(){
    let cli = Cli::parse();

    match get_average(&cli.address, cli.port, cli.ttl, cli.count) {
        Ok(duration) => println!("For {} \n Average is: {duration:?}", cli.address),
        Err(e) => eprintln!("Error: {}", e),
    }
}