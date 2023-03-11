#![feature(proc_macro_hygiene, decl_macro, async_fn_in_trait, read_buf)]

pub mod ftp;
pub mod tcp;
pub mod tls;
pub mod udp;

use crate::tcp::TCPServer;
use anyhow::Result;
use lazy_static::lazy_static;
use spin::RwLock;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use tokio::main;
use tokio::net::TcpStream;
use Open::{Beta, ProcessActive};
/*
测试main函数
 */
#[main]
pub async fn main() -> Result<()> {
    //EE::tcp_server().await?;
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
/// 脚本
pub type Script = Arc<Mutex<Beta<'static, (TcpStream, Arc<RwLock<bool>>), Result<()>>>>;
