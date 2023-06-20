pub mod ipv4 {
    use std::net::{Ipv4Addr, SocketAddrV4};

    #[allow(dead_code)]
    pub async fn local_addr() -> Result<Ipv4Addr, super::error::Error> {
        let socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await?;

        socket.connect("1.1.1.1:80").await?;

        if let std::net::SocketAddr::V4(addr) = socket.local_addr()? {
            Ok(*addr.ip())
        } else {
            Err(super::error::Error::InvalidIpv6Addr)
        }
    }

    #[allow(dead_code)]
    pub fn into_u64(socket: &str) -> Result<u64, super::error::Error> {
        let socket: SocketAddrV4 = socket.parse()?;
        let ip = socket.ip().octets();
        let port = socket.port();

        let mut ipv4_u64 = 0u64;
        ip.into_iter().for_each(|octet| {
            ipv4_u64 = ipv4_u64 << 8 | octet as u64;
        });

        Ok(ipv4_u64 << 16 | port as u64)
    }

    #[allow(dead_code)]
    pub fn from_u64(ipv4_u64: u64) -> Result<SocketAddrV4, super::error::Error> {
        let ip_u32 = ((ipv4_u64 & 0xffffffffffff0000u64) >> 16) as u32;
        let port = (ipv4_u64 & 0xffff) as u16;

        let ip = std::net::Ipv4Addr::new(
            ((ip_u32 & 0xff000000u32) >> 24) as u8,
            ((ip_u32 & 0xff0000u32) >> 16) as u8,
            ((ip_u32 & 0xff00u32) >> 8) as u8,
            (ip_u32 & 0xffu32) as u8,
        );
        Ok(SocketAddrV4::new(ip, port))
    }

    #[cfg(test)]
    mod test {
        use super::{from_u64, into_u64};

        #[test]
        fn ipv4_to_u64() {
            use std::net::{Ipv4Addr, SocketAddrV4};
            let socket = SocketAddrV4::new(Ipv4Addr::new(255, 254, 253, 252), 65535);
            let ipv4_u64 = into_u64(&socket.to_string()).unwrap();
            let ip = from_u64(ipv4_u64).unwrap();

            assert_eq!(ip, socket)
        }
    }
}

pub mod error {

    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("io error: `{0}`")]
        IO(#[from] std::io::Error),
        #[error("invalid ipv6Addr, expected ipv4Addr")]
        InvalidIpv6Addr,
        #[error("invalid addr: parse error: `{0}`")]
        InvalidAddr(#[from] std::net::AddrParseError),
    }

    impl actix_web::ResponseError for Error {
        fn status_code(&self) -> reqwest::StatusCode {
            match self {
                Error::IO(_) | Error::InvalidIpv6Addr => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                Error::InvalidAddr(_) => reqwest::StatusCode::BAD_REQUEST,
            }
        }
    }
}
