#[derive(Debug)]
enum IpAddrKind {
    v4,
    v6,
}
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn main() {
    let home = IpAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };
    println!("{:#?}", home)
}
