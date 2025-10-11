use clap::Parser;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

/// Ping tool written in rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// target address 
    address: String,

    ///Target port. Default is 80
    #[arg(short, long)]
    port: Option<i32>,

    ///Amount of TTL(Time To Live). Default is 117
    #[arg(short, long)]
    ttl: Option<u32>,

    ///Amount of request to send. Default if 5
    #[arg(short, long)]
    count: Option<u32>
}

struct Durations{
    average: Duration,
    least: Duration,
    max: Duration
}

pub struct Ping{
    address: String,
    port: Option<i32>,
    ttl: Option<u32>,
    count: Option<u32>
}

//Public functions
impl Ping{
    pub fn new() -> Self{
        let cli = Cli::parse();
        Ping { 
            address: cli.address, 
            port: cli.port,
            ttl: cli.ttl, 
            count: cli.count 
        }
    }
    pub fn run(&mut self){
        let mut cli = Ping::new();

        match Ping::get_average(&mut cli) {
            Ok(duration) => println!("For {} \n Average: {:?} Least: {:?} Max:{:?}", 
            cli.address, duration.average, duration.least, duration.max),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

//Private functions
impl Ping{
    fn get_average(&mut self) -> Result<Durations, Box<dyn std::error::Error>> {
        let count = self.count.unwrap_or(5);
        let mut durations: Vec<Duration> = Vec::new();
        let mut total: Duration = Duration::ZERO;

        for _ in 0..count {
            let value = Ping::send_request(self)?;
            total += value;
            durations.push(value);
        }

        durations.sort();

        Ok(Durations {
            average: total / count,
            least: *durations.first().unwrap_or(&Duration::ZERO),
            max: *durations.last().unwrap_or(&Duration::ZERO),
        })
    }

    fn send_request(&mut self) -> Result<Duration, Box<dyn std::error::Error>>{
        let now = std::time::Instant::now();
        let port = match self.port{ Some(val) => val, None => 80 };
        let url = format!("{}:{}", self.address, port);

        let connect = TcpStream::connect(&url)?;
        if let Some(val) = self.ttl{
            connect.set_ttl(val)?;
        }else{
            connect.set_ttl(117)?;
        }

        let resolved_addr:Vec<_> = url.to_socket_addrs()?.collect();
        let duration = now.elapsed();

        println!("\tResponse of {} address:{:?}, time={:?} TTL={:?}",
        self.address, 
        resolved_addr,
        duration,
        connect.ttl()?
        );

        std::thread::sleep(std::time::Duration::from_secs(1));
        Ok(duration)
    }
}

