use async_trait::async_trait;
use spin::RwLock;
use std::net;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{BufReader, BufWriter};
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;
use Open::{ProcessActive, ThreadActive};

///# tcp 服务
#[async_trait]
pub trait TCPServer {
    //获取port
    fn get_port() -> SocketAddr;
    async fn connect(stream: TcpStream) -> anyhow::Result<()>;
    async fn server(stream: TcpStream, ae: Arc<RwLock<bool>>) -> anyhow::Result<()>;
    //链接
    async fn tcp_client() -> ThreadActive<()> {
        <Self as TCPServer>::connect(TcpStream::connect(<Self as TCPServer>::get_port()).await?)
            .await?;
        Ok(())
    }
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
                spawn(async move {
                    <Self as TCPServer>::server(tcp, stop)
                        .await
                        .expect("TCP_ASYNC_ERROR");
                });
            }
        }
        Ok(())
    }
}

pub trait TCPServerParallel: TCPServer {
    //链接
    fn connect(_: net::TcpStream) -> ProcessActive<()>;
    fn server(_: net::TcpStream, _: Arc<RwLock<bool>>) -> ProcessActive<()>;
    fn parallel_tcp_client() -> ThreadActive<()> {
        <Self as TCPServerParallel>::connect(net::TcpStream::connect(
            <Self as TCPServer>::get_port(),
        )?)?;
        Ok(())
    }
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
                        <Self as TCPServerParallel>::server(tcp, stop).expect("TCP_PARALLEL_ERROR");
                    });
                }
            }
        };
        Ok(())
    }
}
