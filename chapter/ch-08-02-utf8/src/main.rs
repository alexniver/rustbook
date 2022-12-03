fn main() {
    let mut s = "adasdfasdfa".to_string();
    s.push('c');
    s.push_str("addd");
    println!("string : {:?}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    let s1 = String::from("tic");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let hello = String::from("hello");
    let hello_cn = String::from("你好");
    // let ni = &hello_cn[0];
    //
    for c in hello_cn.chars() {
        println!("c : {}", c);
    }

    for b in hello_cn.bytes() {
        println!("b : {}", b);
    }
}
