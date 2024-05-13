use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng(); // Correctly using rand::thread_rng() from the rand crate
    let port: u16 = rng.gen_range(1024..=65535);
    println!("Random port number: {}", port);
}

