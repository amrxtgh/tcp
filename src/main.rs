use std::io;

/// Creating the user space TCP stack.
/// Bypassing all the operating system built-in TCP stack to directly receive and process the raw packets from the internet.
/// 1st: Setting up the virtual network interface.
/// We employ the TUN(network) - it is the virtual(software based) network interface that exists in the operating systems kernel.
/// It operates on Layer 3 of the OSI model and expose the file descriptor to any application that
/// need to send or receive packets.
fn main() -> io::Result<()> {
    // Creating the new TUN interface named "tun0" in TUN mode.
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;

    // creating the buffer of the size 1504 bytes(maximum Ethernet frame size without CRC) to store the data.
    let mut buf = [0u8; 1504];

    // Loop for continuous receive data from the interface
    loop {
        // receive data from the TUN interface and store the number of bytes in the `nbytes.
        let nbytes = nic.recv(&mut buf[..])?;
        let flags = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);
        if proto != 0x0800 {
            // If the protocol is not IPv4, skip the packet.
            continue;
        }
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(ipv4) => {
                eprintln!(
                    "read {} bytes (flags: {:x?}, proto: {:x?}: {:?})",
                    nbytes - 4,
                    flags,
                    proto,
                    ipv4
                );
            }
            Err(e) => {
                eprintln!("Error parsing IPv4 header: {:?}", e);
            }
        }
    }
    Ok(())
}
