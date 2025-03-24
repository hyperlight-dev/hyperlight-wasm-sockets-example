use crate::bindings::*;

use hyperlight_common::resource::BorrowedResourceGuard;
use wasi::sockets::network::{ErrorCode, IpSocketAddress};
use wasi::sockets::udp::{IncomingDatagram, OutgoingDatagram};

use crate::poll::MyPollable;
use crate::state::MyState;

use std::sync::{Arc, Mutex};

pub struct MyDatagramStream {
    socket: Arc<std::net::UdpSocket>,
}

impl wasi::sockets::udp::IncomingDatagramStream<
    wasi::sockets::network::ErrorCode,
    wasi::sockets::network::IpSocketAddress,
    MyPollable
    >
for MyState {
    type T = MyDatagramStream;

    fn receive(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        _max_results: u64
    ) -> Result<Vec<IncomingDatagram<IpSocketAddress>>, ErrorCode>  {
        let mut buf = vec![0; 65507];
        let (size, addr) = self_.socket.recv_from(&mut buf)
            .map_err(|_| wasi::sockets::network::ErrorCode::Unknown)?;
        Ok(vec![IncomingDatagram {
            data: Vec::from(&buf[0..size]),
            remote_address: std_addr_to_wasi_addr(addr),
        }])
    }

    fn subscribe(
        &mut self,
        _self: BorrowedResourceGuard<Self::T>
    ) -> MyPollable  {
        MyPollable::AlwaysReady
    }
}

impl wasi::sockets::udp::OutgoingDatagramStream<
    wasi::sockets::network::ErrorCode,
    wasi::sockets::network::IpSocketAddress,
    MyPollable
    >
for MyState {
    type T = MyDatagramStream;

    fn check_send(
        &mut self,
        _self: BorrowedResourceGuard<Self::T>
    ) -> std::result::Result<u64, ErrorCode>  {
        Ok(1024)
    }

    fn r#send(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        datagrams: Vec<OutgoingDatagram<IpSocketAddress>>
    ) -> std::result::Result<u64, ErrorCode>  {
        let mut count = 0;
        for datagram in datagrams {
            let Some(addr) = datagram.remote_address else {
                if count > 0 {
                    return Ok(count);
                } else {
                    return Err(wasi::sockets::network::ErrorCode::Unknown)
                }
            };
            let Ok(_) = self_.socket.send_to(&datagram.data, addr) else {
                if count > 0 {
                    return Ok(count);
                } else {
                    return Err(wasi::sockets::network::ErrorCode::Unknown);
                }
            };
            count += 1;
        }
        Ok(count)
    }

    fn r#subscribe(
        &mut self,
        _self: BorrowedResourceGuard<Self::T>
    ) -> MyPollable  {
        MyPollable::AlwaysReady
    }
}

impl wasi::sockets::network::Network for MyState {
    type T = ();
}

pub struct MySocket {
    os: Mutex<Option<Arc<std::net::UdpSocket>>>, // only exists if bound
}

impl std::net::ToSocketAddrs for wasi::sockets::network::IpSocketAddress {
    type Iter = std::iter::Once<std::net::SocketAddr>;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        use wasi::sockets::network::*;
        Ok(std::iter::once(match &self {
            IpSocketAddress::Ipv4(v4) => std::net::SocketAddr::V4(
                std::net::SocketAddrV4::new(
                    std::net::Ipv4Addr::new(
                        v4.address.0, v4.address.1, v4.address.2, v4.address.3
                    ),
                    v4.port
                )
            ),
            IpSocketAddress::Ipv6(_v6) => todo!("ipv6 to_socket_addrs"),
        }))
    }
}

fn std_addr_to_wasi_addr(addr: std::net::SocketAddr) -> wasi::sockets::network::IpSocketAddress {
    use std::net::SocketAddr;
    use wasi::sockets::network::*;
    match addr {
        SocketAddr::V4(v4) => {
            let octets = v4.ip().octets();
            IpSocketAddress::Ipv4(Ipv4SocketAddress {
                port: v4.port(),
                address: (octets[0], octets[1], octets[2], octets[3])
            })
        },
        SocketAddr::V6(_v6) => todo!("ipv6 to wasi addrs")
    }
}

impl wasi::sockets::udp::UdpSocket<
    wasi::sockets::network::ErrorCode,
    MyDatagramStream,
    wasi::sockets::network::IpSocketAddress,
    (),
    MyDatagramStream,
    MyPollable
    >
for MyState {
     type T = MySocket;
    fn start_bind(
        &mut self,
        self_: BorrowedResourceGuard<MySocket>,
        _network: BorrowedResourceGuard<()>,
        local_address: wasi::sockets::network::IpSocketAddress
    ) -> Result<(), wasi::sockets::network::ErrorCode> {
        *(*self_).os.lock().unwrap() = Some(Arc::new(
            std::net::UdpSocket::bind(local_address)
                .map_err(|_| wasi::sockets::network::ErrorCode::Unknown)?));
        Ok(())
    }
    fn stream(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        _remote_address: Option<IpSocketAddress>
    ) -> Result<(MyDatagramStream, MyDatagramStream), ErrorCode> {
        let sock = (*self_).os.lock().unwrap();
        let sock = sock.as_ref().unwrap();
        Ok((MyDatagramStream { socket: sock.clone() },
            MyDatagramStream { socket: sock.clone() }))
    }

    fn finish_bind(
        &mut self,
        _self: BorrowedResourceGuard<Self::T>
    ) -> std::result::Result<(), ErrorCode>  {
        Ok(())
    }

    fn r#subscribe(
        &mut self,
        _self: BorrowedResourceGuard<Self::T>
    ) -> MyPollable  {
        MyPollable::AlwaysReady
    }
}

impl wasi::sockets::UdpCreateSocket<
    wasi::sockets::network::ErrorCode,
    wasi::sockets::network::IpAddressFamily,
    MySocket,
    >
for MyState {
    fn create_udp_socket(
        &mut self,
        _address_family: wasi::sockets::network::IpAddressFamily,
    ) -> Result<MySocket, wasi::sockets::network::ErrorCode> {
        Ok(MySocket {
            os: Mutex::new(None),
        })
    }
}

impl wasi::sockets::Udp<
    ErrorCode,
    IpSocketAddress,
    (),
    MyPollable>
 for MyState {}
