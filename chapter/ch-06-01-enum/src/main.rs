fn main() {
    let ipv4 = IpAddrKind::IPV4;
    let ipv6 = IpAddrKind::IPV6;
    route(ipv4);
    route(ipv6);

    // let home = IpAddr {
    //     kind: IpAddrKind::IPV4,
    //     addr: String::from("127.0.0.1"),
    // };
    //
    // let loop_back = IpAddr {
    //     kind: IpAddrKind::IPV6,
    //     addr: String::from("::1"),
    // };
    //
    // let home = IpAddr::IPV4(String::from("127.0.0.1"));
    // let loop_back = IpAddr::IPV6(String::from("::1"));
    let home = IpAddr::IPV4(127, 0, 0, 1);
    let loop_back = IpAddr::IPV6(String::from("::1"));

    let x: i8 = 5;
    let y: Option<i8> = Some(10);
    // println!("x + y = {:?}", x + y);
    println!("x + y = {:?}", x + y.expect("get i8 error"));
}

#[derive(Debug)]
enum IpAddrKind {
    IPV4,
    IPV6,
}

// struct IpAddr {
//     kind: IpAddrKind,
//     addr: String,
// }
// enum IpAddr {
//     IPV4(String),
//     IPV6(String),
// }

enum IpAddr {
    IPV4(u8, u8, u8, u8),
    IPV6(String),
}
fn route(ipAddrKind: IpAddrKind) {
    println!("ipAddrKind is : {:?}", ipAddrKind);
}
