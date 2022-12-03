use std::io;

fn main() {
    // let guess = "43".parse().expect("not a number str");
    let guess: u32 = "43".parse().expect("not a number str");
    let x = 32.;
    let y: f32 = 44.;
    println!("guess : {:?}", guess);
    println!("x : {:?}", x);
    println!("y : {:?}", y);

    let sum = 5 + 10;
    let sub = 232.3 - 432.9;
    let multi = 23.3 * 53.2;
    let div = 53.53 / 33.51;
    let remainder = 22 % 10;

    println!(
        "sum : {:?}, sub: {:?}, multi: {:?}, div: {:?}, remainder: {:?}",
        sum, sub, multi, div, remainder
    );

    let t = true;
    let f: bool = false;

    println!("t : {:?}, f: {:?}", t, f);

    let c = 'z';
    let c2: char = 'b';
    let emoji = '😻';

    println!("c: {:?}, c2: {:?}, emoji: {:?}", c, c2, emoji);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {:?}, y: {:?}, z: {:?}", x, y, z);

    println!("tup.0: {:?}, tup.1: {:?}, tup.2: {:?}", tup.0, tup.1, tup.2);

    let a = [1, 2, 3, 4, 5];
    println!("array: {:?}", a);

    let str_arr = ["one", "two", "three", "four"];
    println!("str_arr: {:?}", str_arr);

    let b: [u8; 5] = [1, 2, 3, 4, 5];
    println!("b: {:?}", b);

    let c = [3; 5];
    println!("c: {:?}", c);

    let first = c[0];
    let second = c[1];
    println!("first: {:?}, second: {:?}", first, second);

    let a = [1, 2, 3, 4, 5];

    println!("input a index:");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Fail to read line");
    let index: usize = buf.trim().parse().expect("Not a number");
    println!("arr a index : {:?} is : {:?}", index, a[index]);
}
