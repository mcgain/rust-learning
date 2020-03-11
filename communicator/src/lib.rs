#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::client::connect();
    }
}

mod network;

pub mod client;
