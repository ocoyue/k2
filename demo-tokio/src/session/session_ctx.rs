pub enum SessionContext {
    Stream { addr: std::net::SocketAddr },
    File { path: std::path::PathBuf },
    Std,
}
