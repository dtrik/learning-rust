#![allow(unused_variables)]
fn main() {
   
    #[derive(Debug)] 
    //basic enum definition, enumerate all options - called variants
    enum IpAddrBasic {
        V4,
        V6,
    }

    //define two variables using the :: syntax
    let four = IpAddrBasic::V4;
    let six = IpAddrBasic::V6;
	
    //create a more complex datatype that refers to above enum
    #[derive(Debug)]
    struct IpAddrStruct {
        kind: IpAddrBasic,              //enum used as type of kind
        address: String,
    }

    //two variable of type struct defined above
    let home_struct = IpAddrStruct {    
        kind: IpAddrBasic::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback_struct = IpAddrStruct {
        kind: IpAddrBasic::V6,
        address: String::from("::1"),
    };
    
    println!("{:?}", home_struct);
    //function that takes data of type enum
    //note it is generic, it can take either type of IP
    fn route(ip_type: IpAddrBasic) {
    //any process, current noop
    }
    
    //call above function with both kinds
    route(IpAddrBasic::V4);
    route(IpAddrBasic::V6);
	
    //implement same operation as struct with simpler notation
    enum IpAddrKind { //below says both V4 and V6 have assoc. string vals
        V4(String),
        V6(String),
    }

    let home_enum = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback_enum = IpAddrKind::V6(String::from("::1"));
   
}
