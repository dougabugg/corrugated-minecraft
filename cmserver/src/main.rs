use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

// use native_tls::{Identity, TlsAcceptor, TlsStream};

mod mcpdt;

// fn handle_client_tls(mut stream: TlsStream<TcpStream>) {
//     // for now, only handle one client, for simplicity during early development
//     // println!("new client: {:?}", stream.peer_addr()?);
//     println!("new client: {:?}", "stream.peer_addr()?");
//     let mut buf = [0u8; 256];
//     loop {
//         stream.read(&mut buf).unwrap();
//         println!("{:x?}", &buf);
//     }
// }

// pub fn read_handshake_dbg(buffer: &[u8], cursor: &mut usize) -> Option<()> {
//     use mcpdt::*;
//     let mut cur = *cursor;
//     print!("{}, ", cur);
//     let proto_version = read_var_int(buffer, &mut cur)?;
//     print!("{}, ", cur);
//     let server_addr = read_string(buffer, &mut cur)?;
//     print!("{}, ", cur);
//     let server_port = read_u16(buffer, &mut cur)?;
//     print!("{}, ", cur);
//     let next_state = read_var_int(buffer, &mut cur)?;
//     println!("{}", cur);
//     *cursor = cur;
//     Some(())
// }

fn handle_client(mut stream: TcpStream) {
    println!("\nnew client: {:?}", stream.peer_addr().unwrap());
    let mut buf = [0u8; 256];
    let mut cursor = 0usize;
    loop {
        let len = stream.read(&mut buf).unwrap();
        if len == 0 {
            break;
        }
        println!("{:x?}", &buf[cursor..cursor+len]);
        let packet = mcpdt::read_packet(&buf, &mut cursor).unwrap();
        println!("{:?}", packet);
        let hs = mcpdt::read_handshake(&buf, &mut cursor).unwrap();
        // let hs = read_handshake_dbg(&buf, &mut cursor).unwrap();
        println!("{:?}", hs);
        cursor += len;
        let mut packet = vec![];
        let msg = std::fs::read_to_string("assets/msg.json").ok().unwrap();
        mcpdt::write_string(&msg, &mut packet).unwrap();
        mcpdt::write_packet(&packet, 0, &mut stream).unwrap();
        break;
    }
    println!("client disconnected");
}

fn main() {
    // let mut file = std::fs::File::open("assets/sample_identity.pfx").unwrap();
    // let mut identity = vec![];
    // file.read_to_end(&mut identity).unwrap();
    // let identity = Identity::from_pkcs12(&identity, "").unwrap();
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();
    // let acceptor = TlsAcceptor::new(identity).unwrap();
    // accept connections and process them serially
    for stream in listener.incoming() {
        // handle_client(acceptor.accept(stream.unwrap()).unwrap());
        handle_client(stream.unwrap());
    }
}
