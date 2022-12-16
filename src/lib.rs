#![cfg_attr(not(test), no_std)]
//#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod tcp;
//mod udp;
pub mod addr;
pub mod dns;

use tcp::TcpStack;

use crate::dns::{Dns, DnsError};
use crate::tcp::TcpError;
use core::fmt::Debug;

/// Aggregation and homegenation of IP network stacks
pub trait IpNetworkDriver {
	type TcpSocket;
	type TcpError: Into<TcpError> + Debug;

	type DnsError: Into<DnsError> + Debug;

	//type UdpSocket;
	//type UdpError: Into<UdpError> + Debug;

	fn tcp(&self) -> &dyn TcpStack<TcpSocket = Self::TcpSocket, Error = Self::TcpError>;
	//fn udp() -> &dyn UdpStack<Error = NetworkError>;
	fn dns(&self) -> &dyn Dns<Error = Self::DnsError>;
}
