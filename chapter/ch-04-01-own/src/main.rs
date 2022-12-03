use std::str::FromStr;

fn main() {
    {
        let s = "hello";
    }
    {
        let mut s = String::from("hello");
        s.push_str(", world");
    }
    {
        let s1 = String::from("hello");
        let s2 = s1;
        // println!("{}", s1);
    }

    {
        let y = 5;
        let x = y;
        println!("y: {y}, x: {x}");
    }

    {
        let s = String::from("Hello");
        take_ownerships(s);
        // let s1 = s;
    }

    {
        let i = 3;
        makes_copy(i);
        let v = i;
        println!("v: {v}");
    }

    {
        let s = gives_ownership();
        let s3 = take_and_giveback_ownership(s);
        // println!("s: {s}");
        println!("s3: {s3}");
    }

    {
        let s = String::from("Helloooo");
        let res = calculate_len(s);
        println!("s: {}, len: {}", res.0, res.1);
    }
}

fn take_ownerships(s: String) {
    println!("take_ownerships: {s}");
}

fn makes_copy(i: i32) {
    println!("makes_copy: {i}");
}

fn gives_ownership() -> String {
    let s = String::from("asfasdfasdf");
    s
}

fn take_and_giveback_ownership(s: String) -> String {
    s
}

fn calculate_len(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
