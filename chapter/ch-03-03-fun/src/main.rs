fn main() {
    println!("Hello, world!");
    a_function();

    print_num(23);

    print_measure(33, 'b');

    statement();

    print_num(five());

    print_num(plus_one(2));
}

fn a_function() {
    println!("a function");
}

fn print_num(x: u32) {
    println!("num: {:?}", x);
}

fn print_measure(n: u32, label: char) {
    println!("measure: {:?}{:?}", n, label);
}

fn statement() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("statement: {:?}", y);
}

fn five() -> u32 {
    5
}

fn plus_one(n: u32) -> u32 {
    n + 1
}
