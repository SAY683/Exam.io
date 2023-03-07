#![feature(proc_macro_hygiene, decl_macro)]

pub mod tcp;
pub mod udp;

use anyhow::Result;
use lazy_static::lazy_static;
use std::net::{SocketAddr, UdpSocket};
use tokio::main;
/*
测试main函数
 */
#[main]
pub async fn main() -> Result<()> {
    Ok(())
}
lazy_static! {
    pub static ref LOCAL_IP: anyhow::Result<String> = {
        let x = UdpSocket::bind("0.0.0.0:0")?;
        x.connect("8.8.8.8:80")?;
        Ok(x.local_addr()?.ip().to_string())
    };
    pub static ref LOCAL_HOST: anyhow::Result<SocketAddr> = {
        let x = UdpSocket::bind("0.0.0.0:0")?;
        x.connect("8.8.8.8:80")?;
        Ok(x.local_addr()?)
    };
    pub static ref LOCAL_PORT: anyhow::Result<String> = {
        let x = UdpSocket::bind("0.0.0.0:0")?;
        x.connect("8.8.8.8:80")?;
        Ok(x.local_addr()?.port().to_string())
    };
}
