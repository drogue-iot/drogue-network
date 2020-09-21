//! # embedded-nal - A Network Abstraction Layer for Embedded Systems

#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod dns;
pub use dns::{Dns, AddrType};

pub use embedded_nal::{
	Mode,
	TcpStack,
	UdpStack,
	IpAddr,
	Ipv4Addr,
	Ipv6Addr,
	SocketAddr,
	SocketAddrV4,
	SocketAddrV6,
};


