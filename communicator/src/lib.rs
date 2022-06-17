// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]
pub mod client; // contains no submodules, place in client.rs

// network contains submodules, move the declarations of network
// to own directory, network/mod.rs or else Rust will not know
// where to look for network's submodules
// declare network::connect() inside mod.rs and declare mod server;
// declare network::server inside network/server.rs
mod network;

// Notice there's no main.rs
// Leave this at the bottom of lib.rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
