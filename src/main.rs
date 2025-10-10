#[path ="ping.rs"]
mod ping;
use ping::Ping;

fn main(){
    let mut ping = Ping::new().run();
}
