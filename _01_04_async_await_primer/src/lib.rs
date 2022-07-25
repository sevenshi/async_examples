#![cfg(test)]
use std::{time::Duration, thread::sleep};
use futures::executor::block_on;

mod first {
    use  futures::executor::block_on;
    async fn hello_world() {
        println!("hello, world!");
    }
    
    pub(crate) fn main() {
        let future = hello_world(); // Nothing is printed
        block_on(future); // `future` is run and "hello, world!" is printed
    }
    // ANCHOR_END: hello_world
    
    #[test]
    fn run_main() { main() }
}

struct Song;
async fn learn_song() -> Song { 
    sleep(Duration::from_secs(3));
    println!("learn_song done!!");
    Song 
}
async fn sing_song(_: Song) {
    sleep(Duration::from_secs(3));
    println!("sing_song done!!");
}
async fn dance() {
    sleep(Duration::from_secs(3));
    println!("dance done!!");
}

mod second {
use super::*;
// ANCHOR: block_on_each
fn main() {
    let song = block_on(learn_song());
    
    block_on(sing_song(song));
   
    block_on(dance());

}

    #[test]
    fn run_main() { main() }
}

mod third {

    use super::*;
    // ANCHOR: block_on_main
    async fn learn_and_sing() { 
        let song = learn_song().await;
        sing_song(song).await;
    }

    async fn async_main() {
        let f1 = learn_and_sing();
        let f2 = dance();
        futures::join!(f1, f2);    
    }

    fn main() {
        block_on(async_main());
    }

    #[test]
    fn run_main() { main() }
}