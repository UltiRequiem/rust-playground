#[derive(Debug)]
struct Ipv4Addr;

#[derive(Debug)]
struct Ipv6Addr;

#[derive(Debug)]
enum IpAddrKind {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl IpAddr {
    /// Returns a new Instance of IpAddr
    fn new(kind: IpAddrKind, address: &str) -> IpAddr {
        IpAddr {
            kind,
            address: address.to_string(),
        }
    }
}

fn main() {
    let home = IpAddr::new(IpAddrKind::V4(Ipv4Addr), "127.0.0.1");
    let loopback = IpAddr::new(IpAddrKind::V6(Ipv6Addr), "::1");

    println!("{:?}", home);
    println!("{:?}", loopback);
}
