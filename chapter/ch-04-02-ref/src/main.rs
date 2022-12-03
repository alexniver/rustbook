fn main() {
    {
        let mut s = String::from("adaafsadfasdfasdf");
        println!("s len : {}", calculate_len(&s));
        change(&mut s);
        println!("s len : {}", calculate_len(&s));
    }

    {
        let mut s = String::from("Hello");
        let s1 = &mut s;
        // let s2 = &mut s;
        // println!("{}, {}", s1, s2);
    }
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("hello");
}
