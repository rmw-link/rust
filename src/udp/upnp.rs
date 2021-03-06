use async_std::task::sleep;
use igd::aio::search_gateway;
use igd::AddPortError::PortInUse;
use log::info;
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};
use std::time::Duration;

pub async fn upnp(name: &str, port: u16, duration: u32) -> Option<(SocketAddrV4, Ipv4Addr)> {
  if let Ok(gateway) = search_gateway(Default::default()).await {
    let gateway_addr = gateway.addr;
    if let Ok(stream) = TcpStream::connect(gateway_addr) {
      if let Ok(addr) = stream.local_addr() {
        let ip = addr.ip();
        drop(stream);
        if let IpAddr::V4(ip) = ip {
          let mut retry = true;
          loop {
            match gateway
              .add_port(
                igd::PortMappingProtocol::UDP,
                port,
                SocketAddrV4::new(ip, port),
                duration,
                name,
              )
              .await
            {
              Err(err) => {
                match err {
                  PortInUse => {
                    if retry {
                      retry = false;
                      match gateway
                        .remove_port(igd::PortMappingProtocol::UDP, port)
                        .await
                      {
                        Err(err) => info!("upnp remove port {} error {}", port, err),
                        Ok(_) => {
                          continue;
                        }
                      }
                    }
                  }
                  _ => {}
                }
                info!("upnp failed {} > {}", gateway_addr, err)
              }
              Ok(_) => {
                return Some((gateway_addr, ip));
              }
            }
          }
        }
      }
    }
  }
  None
}

const SLEEP_SECONDS: u32 = 60;

pub async fn upnp_daemon(name: &str, port: u16) {
  let mut local_ip = Ipv4Addr::UNSPECIFIED;
  let mut pre_gateway = SocketAddrV4::new(local_ip, 0);

  let seconds = Duration::from_secs(SLEEP_SECONDS.into());

  loop {
    if let Some((gateway, ip)) = upnp(name, port, 0).await {
      if ip != local_ip || gateway != pre_gateway {
        local_ip = ip;
        pre_gateway = gateway;
        info!("upnp success ( addr {}:{} ; gateway {})", ip, port, gateway);
      }
    };
    sleep(seconds).await;
  }
}
