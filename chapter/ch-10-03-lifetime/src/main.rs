fn main() {
    let s1 = String::from("12345");
    let result;
    {
        let s2 = String::from("123456");
        result = longest(s1.as_str(), s2.as_str());
        println!("longest : {}", result);
    }
    // println!("longest : {}", result);

    let item;
    {
        let novel = String::from("Many many years ago, there is a little forg");
        item = Item {
            text: novel.as_str(),
        };
        println!("Item: {:?}", item);
    }
    // println!("Item: {:?}", item);
}

#[derive(Debug)]
struct Item<'a> {
    text: &'a str,
}

// struct Item2 {
//     text: &str,
// }

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
