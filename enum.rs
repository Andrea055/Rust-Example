enum IP {
    V4(String),
    V6(String)
}

fn main(){
    let localhost = IP::V4(String::from("127.0.0.1"));
    let localhost6 = IP::V6(String::from("::1"));
}
