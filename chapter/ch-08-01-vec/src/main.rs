fn main() {
    let mut v = vec![];

    v.push(2);
    v.push(5);
    v.push(8);

    let third = &v[2];
    // v.push(9);
    println!("third is : {:?}", third);

    // let fourth = &v[3];
    //

    let fourth = v.get(3);
    match fourth {
        Some(n) => println!("fourth is : {:?}", n),
        None => println!("fourth is none"),
    }

    for n in &mut v {
        *n += 1;
    }
    for n in &v {
        println!("loop : {:?}", n);
    }

    let cell_arr = vec![
        Cell::Int(2),
        Cell::Float(3.2),
        Cell::Str(String::from("232rf")),
    ];
}

enum Cell {
    Int(i32),
    Float(f32),
    Str(String),
}
