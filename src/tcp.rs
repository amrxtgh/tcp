pub struct State {}

impl Default for State {
    fn default() -> Self {
        State {}
    }
}

impl State {
    pub fn on_packet<'a>(
        &mut self,
        ipv4h: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) {
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
