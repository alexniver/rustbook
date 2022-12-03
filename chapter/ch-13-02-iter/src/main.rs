fn main() {
    let v1 = vec![1, 2, 3, 4];
    let v1_iter = v1.iter();
    for v in v1_iter {
        println!("v: {v}");
    }
}

#[test]
fn iter_test() {
    let v1 = vec![1, 2, 3, 4];
    let mut v_iter = v1.iter();

    assert_eq!(v_iter.next(), Some(&1));
    assert_eq!(v_iter.next(), Some(&2));
    assert_eq!(v_iter.next(), Some(&3));
    assert_eq!(v_iter.next(), Some(&4));
    assert_eq!(v_iter.next(), None);
}

#[test]
fn iter_sum_test() {
    let v1 = vec![1, 2, 3, 4, 5];
    // let mut v_iter = v1.iter();
    // let total: i32 = v_iter.sum();
    let total = v1.iter().sum();
    assert_eq!(15, total);
}

#[test]
fn iter_map_test() {
    let v1 = vec![1, 2, 3, 4, 5];
    let mut m = v1.iter().map(|x| x + 1);
    assert_eq!(m.next(), Some(2));
    assert_eq!(m.next(), Some(3));
    assert_eq!(m.next(), Some(4));
    assert_eq!(m.next(), Some(5));
    assert_eq!(m.next(), Some(6));
    assert_eq!(m.next(), None);
}

#[test]
fn iter_collect_test() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|shoe| shoe.size == shoe_size)
        .collect()
}

#[test]
fn shoe_filter_test() {
    let shoe_arr = vec![
        Shoe {
            size: 23,
            style: String::from("red"),
        },
        Shoe {
            size: 24,
            style: String::from("blue"),
        },
        Shoe {
            size: 23,
            style: String::from("red"),
        },
    ];

    let shoe_filter_arr = shoes_in_size(shoe_arr, 23);

    assert_eq!(shoe_filter_arr.len(), 2);
}
