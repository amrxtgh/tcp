use std::io;

/// We are going to create the user space TCP stack.
/// We have to bypass all the operating system built-in TCP stack to directly receive and process
/// the raw packets from the internet.
/// 
/// 1st: we are going to set up the virtual network interface. 
/// We employ the TUN(network) - it is the virtual(software based) network interface that exists in
/// the operating systems kernel.
/// It operates on Layer 3 of the OSI model and expose the file descriptor to any application that
/// need to send or receive packets.
fn main() -> io::Result<()> {
    // Creating the new TUN interface named "tun0" in TUN mode.
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;

    // creating the buffer of the size 1504 bytes(maximum Ethernet frame size without CRC) to store the data.
    let mut buf = [0u8; 1504];


    // Loop for continuous receive data from the interface
    loop {
        // receive data from the TUN interface and store the number of bytes in the `nbytes`.
        let nbytes = nic.recv(&mut buf[..])?;

        eprintln!("read {} bytes: {:x?}", nbytes, &buf[..nbytes]);
    }
    Ok(())
}
