use std::net::SocketAddr;
use tokio::{io, spawn};
use tokio::net::{TcpListener, TcpStream};

pub(crate) async fn init_tcp() -> io::Result<TcpListener> {
    TcpListener::bind(("127.0.0.1", 9000)).await
}
pub(crate) async fn tcp_loop<F, Fut>(
    tcp_listener: TcpListener,
    handler: F,
) -> io::Result<()>
where
    F: Fn(TcpStream, SocketAddr) -> Fut + Clone + Send + 'static,
    Fut: Future<Output = io::Result<()>> + Send + 'static,
{
    loop {
        let (stream, addr) = tcp_listener.accept().await?;
        let handler = handler.clone();
        spawn(async move {
            if let Err(e) = handler(stream, addr).await {
                eprintln!("{}", e);
            }
        });
    }
}
