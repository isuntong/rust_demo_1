#![feature(impl_trait_in_assoc_type)]

use std::net::SocketAddr;

use rust_demo_1::{S};

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    volo_gen::rust_demo_1::ItemServiceServer::new(S)
        .run(addr)
        .await
        .unwrap();
}
