fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("favorite_color : {color}");
    } else if is_tuesday {
        println!("today is tuesday");
    } else if let Ok(age) = age {
        println!("age is : {age}");
    } else {
        println!("default!!");
    }

    let mut stack = vec![];
    stack.push(0);
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(n) = stack.pop() {
        println!("while let iter num : {n}");
    }

    let mut v = vec![];
    v.push('a');
    v.push('b');
    v.push('c');

    for (i, v) in v.iter().enumerate() {
        println!("for loop by index {i}, v: {v}");
    }

    let (x, y, z) = (2, 3, 4);
    println!("x: {x}, y: {y}, z: {z}");

    print_coord(&(1, 2, 3));
}

fn print_coord(&(x, y, z): &(i32, i32, i32)) {
    println!("coord: x: {x}, y: {y}, z: {z}");
}
