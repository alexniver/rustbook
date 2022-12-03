fn main() {
    let s = String::from("Hello, world!");
    let first_word = first_word(&s);
    let second_word = second_word(&s);
    println!("first_word: {}", first_word);
    println!("second_world: {}", second_word);

    let a = [1, 2, 3, 4, 5];
    assert_eq!(&a[1..3], &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut second_start = -1;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if second_start == -1 && i + 1 < s.len() - 1 {
                second_start = (i + 1) as i32;
            } else {
                return &s[second_start as usize..i];
            }
        }
    }
    if second_start == -1 {
        second_start = 0;
    }

    &s[second_start as usize..]
}
