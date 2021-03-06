//! UDP socket with flow statistic monitored

use std::{io, net::SocketAddr, sync::Arc};

use shadowsocks::{
    relay::{socks5::Address, udprelay::options::UdpSocketControlData},
    ProxySocket,
};
use tokio::net::ToSocketAddrs;

use super::flow::FlowStat;

/// Monitored `ProxySocket`
pub struct MonProxySocket {
    socket: ProxySocket,
    flow_stat: Arc<FlowStat>,
}

impl MonProxySocket {
    /// Create a new socket with flow monitor
    pub fn from_socket(socket: ProxySocket, flow_stat: Arc<FlowStat>) -> MonProxySocket {
        MonProxySocket { socket, flow_stat }
    }

    /// Send a UDP packet to addr through proxy
    #[inline]
    pub async fn send(&self, addr: &Address, payload: &[u8]) -> io::Result<()> {
        let n = self.socket.send(addr, payload).await?;
        self.flow_stat.incr_tx(n as u64);

        Ok(())
    }

    /// Send a UDP packet to addr through proxy
    #[inline]
    pub async fn send_with_ctrl(
        &self,
        addr: &Address,
        control: &UdpSocketControlData,
        payload: &[u8],
    ) -> io::Result<()> {
        let n = self.socket.send_with_ctrl(addr, control, payload).await?;
        self.flow_stat.incr_tx(n as u64);

        Ok(())
    }

    /// Send a UDP packet to target from proxy
    #[inline]
    pub async fn send_to<A: ToSocketAddrs>(&self, target: A, addr: &Address, payload: &[u8]) -> io::Result<()> {
        let n = self.socket.send_to(target, addr, payload).await?;
        self.flow_stat.incr_tx(n as u64);

        Ok(())
    }

    /// Send a UDP packet to target from proxy
    #[inline]
    pub async fn send_to_with_ctrl<A: ToSocketAddrs>(
        &self,
        target: A,
        addr: &Address,
        control: &UdpSocketControlData,
        payload: &[u8],
    ) -> io::Result<()> {
        let n = self.socket.send_to_with_ctrl(target, addr, control, payload).await?;
        self.flow_stat.incr_tx(n as u64);

        Ok(())
    }

    /// Receive packet from Shadowsocks' UDP server
    ///
    /// This function will use `recv_buf` to store intermediate data, so it has to be big enough to store the whole shadowsocks' packet
    ///
    /// It is recommended to allocate a buffer to have at least 65536 bytes.
    #[inline]
    pub async fn recv(&self, recv_buf: &mut [u8]) -> io::Result<(usize, Address)> {
        let (n, addr, recv_n) = self.socket.recv(recv_buf).await?;
        self.flow_stat.incr_rx(recv_n as u64);

        Ok((n, addr))
    }

    /// Receive packet from Shadowsocks' UDP server
    ///
    /// This function will use `recv_buf` to store intermediate data, so it has to be big enough to store the whole shadowsocks' packet
    ///
    /// It is recommended to allocate a buffer to have at least 65536 bytes.
    #[inline]
    pub async fn recv_with_ctrl(
        &self,
        recv_buf: &mut [u8],
    ) -> io::Result<(usize, Address, Option<UdpSocketControlData>)> {
        let (n, addr, recv_n, control) = self.socket.recv_with_ctrl(recv_buf).await?;
        self.flow_stat.incr_rx(recv_n as u64);

        Ok((n, addr, control))
    }

    /// Receive packet from Shadowsocks' UDP server
    ///
    /// This function will use `recv_buf` to store intermediate data, so it has to be big enough to store the whole shadowsocks' packet
    ///
    /// It is recommended to allocate a buffer to have at least 65536 bytes.
    #[inline]
    pub async fn recv_from(&self, recv_buf: &mut [u8]) -> io::Result<(usize, SocketAddr, Address)> {
        let (n, peer_addr, addr, recv_n) = self.socket.recv_from(recv_buf).await?;
        self.flow_stat.incr_rx(recv_n as u64);

        Ok((n, peer_addr, addr))
    }

    /// Receive packet from Shadowsocks' UDP server
    ///
    /// This function will use `recv_buf` to store intermediate data, so it has to be big enough to store the whole shadowsocks' packet
    ///
    /// It is recommended to allocate a buffer to have at least 65536 bytes.
    #[inline]
    pub async fn recv_from_with_ctrl(
        &self,
        recv_buf: &mut [u8],
    ) -> io::Result<(usize, SocketAddr, Address, Option<UdpSocketControlData>)> {
        let (n, peer_addr, addr, recv_n, control) = self.socket.recv_from_with_ctrl(recv_buf).await?;
        self.flow_stat.incr_rx(recv_n as u64);

        Ok((n, peer_addr, addr, control))
    }

    #[inline]
    pub fn get_ref(&self) -> &ProxySocket {
        &self.socket
    }

    #[inline]
    pub fn flow_stat(&self) -> &FlowStat {
        &self.flow_stat
    }
}
