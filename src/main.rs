use argparse::{ArgumentParser, Store};
use rdp::core::client::Connector;
use std::net::{SocketAddr, TcpStream};

fn main() {
    let mut connect: String = "127.0.0.1:3389".to_owned();
    let mut username: String = "Administrator".to_owned();
    let mut password: String = "".to_owned();
    let mut domain: String = "".to_owned();

    {
        let mut parser = ArgumentParser::new();
        parser
            .refer(&mut connect)
            .add_option(&["--connect"], Store, "");
        parser
            .refer(&mut username)
            .add_option(&["--username"], Store, "");
        parser
            .refer(&mut password)
            .add_option(&["--password"], Store, "");
        parser
            .refer(&mut domain)
            .add_option(&["--domain"], Store, "");
        parser.parse_args_or_exit();
    }

    let mut connector = Connector::new()
        .screen(1280, 1024)
        .credentials(domain, username, password)
        .blank_creds(false)
        .auto_logon(true)
        .check_certificate(false)
        .name("Dummp RDP Client".to_string())
        .use_nla(true);

    let addr = connect.parse::<SocketAddr>().unwrap();
    let tcp = TcpStream::connect(addr).unwrap();
    println!("Connected!");
    let mut client = connector.connect(tcp).unwrap();
    println!("Handshaked!");
    loop {
        client.read(|_| {}).unwrap();
    }
}
