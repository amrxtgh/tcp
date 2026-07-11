use etherparse::Ethernet2Header;

#[Default]
pub enum State {
    Closed,
    Listen,
    SynRcvd,
    Estab,
}

//impl Default for State {
//    fn default() -> Self {
//        State {}
//    }
//}
impl State {
    pub fn on_packet<'a>(
        &mut self,
        ipv4h: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) {
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
                let ack = tcph.sequence_number().wrapping_add(1) as u16;
                let mut syn_ack = etherparse::TcpHeader::new(
                    tcph.destination_port(),
                    tcph.source_port(),
                    seq,
                    ack,
                );
                syn_ack.syn = true;
                syn_ack.ack = true;
                let mut ip = etherparse::Ipv4Header::new(
                    syn_ack.slice().len(),
                    64,
                    etherparse::IpTrafficClass::TCP,
                    ipv4h.destination_addr(),
                    ipv4h.source_addr(),
                );
            }
        }
        eprintln!(
            "{}: {} -> {}:{} {}b of tcp",
            ipv4h.source_addr(),
            tcph.source_port(),
            ipv4h.destination_addr(),
            tcph.destination_port(),
            data.len(),
        )
    }
}
