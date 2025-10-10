#[path ="ping.rs"]
mod ping;
use ping::Ping;

fn main(){
    Ping::new().run();
}
