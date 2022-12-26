pub mod client;

pub mod network;

pub fn message() {
  println!("some string from library");
}

#[cfg(test)]
mod tests {
  use super::client;

  #[test]
  fn it_works() {
    client::connect();
  }
}