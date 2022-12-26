use communicator;

pub mod a {
    pub mod series {
        pub mod of {
            pub fn one_module() {}
            pub fn two_module() {}
        }
    }
}

use a::series::of::{one_module, two_module};

fn main() {
    communicator::client::connect();
    one_module();
    println!("Hello, world!");
}
