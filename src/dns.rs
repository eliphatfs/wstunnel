use hickory_resolver::TokioAsyncResolver;
use std::net::{IpAddr, SocketAddr, SocketAddrV4, SocketAddrV6};
use tracing::trace;

#[derive(Clone)]
pub enum DnsResolver {
    System,
    TrustDns(TokioAsyncResolver),
}

impl DnsResolver {
    pub async fn lookup_host(&self, domain: &str, port: u16) -> anyhow::Result<Vec<SocketAddr>> {
        trace!("-- DnsResolver DNS lookup! resolver {:?} host {:?}:{:?}", match self {
            Self::System => "SYSTEM",
            Self::TrustDns(_dns_resolver) => "TRUST"
        }, domain, port);
        let addrs: Vec<SocketAddr> = match self {
            Self::System => tokio::net::lookup_host(format!("{}:{}", domain, port)).await?.collect(),
            Self::TrustDns(dns_resolver) => dns_resolver
                .lookup_ip(domain)
                .await?
                .into_iter()
                .map(|ip| match ip {
                    IpAddr::V4(ip) => SocketAddr::V4(SocketAddrV4::new(ip, port)),
                    IpAddr::V6(ip) => SocketAddr::V6(SocketAddrV6::new(ip, port, 0, 0)),
                })
                .collect(),
        };

        Ok(addrs)
    }
}
