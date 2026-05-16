use std::net::SocketAddr;
use tokio::{io, spawn};
use tokio::net::{TcpListener, TcpStream};

pub(crate) async fn init_tcp() -> io::Result<TcpListener> {
    TcpListener::bind("127.0.0.1:9000").await
}
pub(crate) async fn run_tcp_loop<F, Fut>(
    listener: TcpListener,
    closure: F,
) -> io::Result<()>
where
    F: Fn(TcpStream, SocketAddr) -> Fut + Send + 'static,
    Fut: Future<Output = io::Result<()>> + Send + 'static,
{
    loop {
        match listener.accept().await {
            Ok((stream,  addr)) => {
                let fut = closure(stream, addr);
                spawn(async move {
                    if let Err(e) = fut.await {
                        eprintln!("session error: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("accept error: {}", e);
                continue;
            }
        }
    }
}

