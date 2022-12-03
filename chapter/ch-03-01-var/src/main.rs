fn main() {
    let mut x = 4;
    x = x + 1;

    println!("x : {:?}", x);

    let x = x + 1;

    {
        let x = x * x;
        println!("inner x : {:?}", x);
    }

    println!("outer x : {:?}", x);

    let spaces = "    ";
    let spaces = spaces.len();
}
