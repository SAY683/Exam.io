use futures::executor::block_on;
use spin::RwLock;
use std::net;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use Open::{ProcessActive, ThreadActive};

///# tcp 服务
pub trait TCPServer {
    //获取port
    fn get_port() -> SocketAddr;
    //处理
    async fn server(_: TcpStream, _: Arc<RwLock<bool>>) -> ProcessActive<()>;
    //服务 阻塞函数
    async fn tcp_server() -> ThreadActive<()> {
        let etc = TcpListener::bind(<Self as TCPServer>::get_port()).await?;
        let config = Arc::new(RwLock::new(false));
        'alp: loop {
            if *config.read() {
                break 'alp;
            } else {
                let stop = config.clone();
                let tcp = etc.accept().await?.0;
                tokio::spawn(
                    async move { block_on(<Self as TCPServer>::server(tcp, stop)).unwrap() },
                );
            }
        }
        Ok(())
    }
}

pub trait TCPServerParallel: TCPServer {
    //处理
    fn server(_: net::TcpStream, _: Arc<RwLock<bool>>) -> ProcessActive<()>;
    fn parallel_tcp_server() -> ThreadActive<()> {
        let config = Arc::new(RwLock::new(false));
        let e = net::TcpListener::bind(<Self as TCPServer>::get_port())?;
        'alp: {
            for e in e.incoming() {
                if *config.read() {
                    break 'alp;
                } else {
                    let tcp = e?;
                    let stop = config.clone();
                    rayon::spawn(move || {
                        <Self as TCPServerParallel>::server(tcp, stop).unwrap();
                    });
                }
            }
        };
        Ok(())
    }
}
