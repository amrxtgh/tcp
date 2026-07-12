use etherparse::IpNumber;

pub enum State {
    Closed,
    Listen,
    SynRcvd,
    Estab,
}

impl Default for State {
    fn default() -> Self {
        State::Closed
    }
}

impl State {
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        ipv4h: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) {
        let mut buf = [0u8; 1500];
        match *self {
            State::Closed => {
                return;
            }
            State::Listen => {
                if !tcph.syn() {
                    // only expected SYN packet
                    return;
                }
                let seq = 0;
                let ack = tcph.sequence_number().wrapping_add(1);
                let mut syn_ack = etherparse::TcpHeader::new(
                    tcph.destination_port(),
                    tcph.source_port(),
                    seq,
                    1500,
                );
                syn_ack.syn = true;
                syn_ack.ack = true;
                syn_ack.acknowledgment_number = ack;
                let tcp_len = syn_ack.header_len();
                let mut ip = etherparse::Ipv4Header::new(
                    tcp_len as u16,
                    64,
                    IpNumber::TCP,
                    ipv4h.destination_addr().octets(),
                    ipv4h.source_addr().octets(),
                )
                .unwrap();
                ip.header_checksum = ip.calc_header_checksum();
                let mut unwritten = &mut buf[..];
                ip.write(&mut unwritten).unwrap();
                syn_ack.write(&mut unwritten).unwrap();
                let written = 1500 - unwritten.len();
                nic.send(&buf[..written]).unwrap();
            }
            _ => {}
        }
    }
}
