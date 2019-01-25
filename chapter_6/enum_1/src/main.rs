#![allow(unused_variables)]
fn main() {

    enum IpAddrBasic {
        V4,
        V6,
    }

    struct IpAddrStruct {
        kind: IpAddrBasic,
        address: String,
    }
    
    let home_struct = IpAddrStruct {
        kind: IpAddrBasic::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback_struct = IpAddrStruct {
        kind: IpAddrBasic::V6,
        address: String::from("::1"),
    };
    
 
    enum IpAddrKind {
        V4(String),
        V6(String),
    }

    let home_enum = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback_enum = IpAddrKind::V6(String::from("::1"));
   
    fn route(ip_type: IpAddrBasic) {}
    route(IpAddrBasic::V4);
    route(IpAddrBasic::V6);
	
    let four = IpAddrBasic::V4;
    let six = IpAddrBasic::V6;
	
}
