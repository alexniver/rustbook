fn main() {
    let x = 1;

    let s = match x {
        1 => "one",
        2 => "two",
        _ => "others",
    };

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("got 50"),
        Some(y) => println!("got y: {y}"),
        _ => println!("Default, x: {:?}", x),
    }

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("others"),
    }

    let x = -1;

    match x {
        1..=5 => println!("less equal 5, bigger equal 1"),
        5.. => println!("bigger than 5"),
        _ => println!("less than 1"),
    }

    let x = 'z';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("later ASCII letter"),
        _ => println!("others"),
    }

    let p = Point { x: 10, y: 20 };

    let Point { x, y } = p;
    println!("point x: {x}, y: {y}");

    let p = Point { x: 0, y: 5 };
    match p {
        Point { x: 0, y } => println!("x is zero, and y: {y}"),
        Point { x, y: 0 } => println!("y is zero, and x: {x}"),
        _ => println!("x: {x}, y: {y}"),
    }

    // let msg = Message::ChangeColor(2, 33, 66);
    let msg = Message::ShowColor(Color::Hsv(33, 55, 44));

    match msg {
        Message::Quit => println!("bye"),
        Message::Move { x, y } => println!("move to x: {x}, y: {y}"),
        Message::Write(msg) => println!("message: {msg}"),
        Message::ChangeColor(r, g, b) => println!("color: {r}, {g}, {b}"),
        Message::ShowColor(c) => match c {
            Color::Hsv(h, s, v) => println!("hsv color: {h} {s} {v}"),
            Color::Rgb(r, g, b) => println!("rgb color: {r} {g} {b}"),
        },
    }

    let ((feet, intch), Point { x, y }) = ((22, 33), Point { x: 333, y: 444 });

    foo(3, 4);

    let v1 = Some(2);
    let v2 = Some(4);

    match (v1, v2) {
        (None, None) => println!("none, none"),
        (None, Some(y)) => println!("none, {y}"),
        (Some(x), None) => println!("{x}, none"),
        (Some(x), Some(y)) => println!("{x}, {y}"),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (a, b, _, _, e) => println!("{a}, {b}, _, _, {e}"),
        _ => println!("others"),
    }

    let s = Some(String::from("Hello!"));
    // if let Some(_s) = s {
    if let Some(_) = s {
        println!("matched!");
    }
    println!("s: {:?}", s);

    let p = Point { x: 10, y: 200 };

    match p {
        Point { x, .. } => println!("x is {x}"),
    }
    let Point { x: _, y } = p;
    println!("point _, y: {y}");

    let num = Some(5);

    match num {
        Some(x) if x % 2 == 0 => println!("{x} is even"),
        Some(x) => println!("{x} is odd"),
        None => println!("nan"),
    }

    let x = Some(10);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched! n == y, which is {n}"),
        _ => println!("Default, x: {:?}", x),
    }

    let x = 4;
    let y = true;

    match x {
        4 | 5 | 6 if y => println!("4 or 5 or 6 and y is yes"),
        _ => println!("no"),
    }

    let msg = Msg::Hello { id: 22 };
    match msg {
        Msg::Hello {
            id: id_alias @ 3..=7,
        } => println!("id in range 3-7 {}", id_alias),

        Msg::Hello {
            id: id_alias @ 8..=15,
        } => println!("id in range 8-15 {}", id_alias),

        Msg::Hello { id: id_alias } => {
            println!("id_alias: {}", id_alias);
        }
    }

    let msgb = MsgBool::Hello { cond: false };
    match msgb {
        MsgBool::Hello { cond: c @ false } => println!("{c}"),
        MsgBool::Hello { cond: c @ true } => println!("{c}"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    ShowColor(Color),
}

enum Color {
    Hsv(i32, i32, i32),
    Rgb(i32, i32, i32),
}

fn foo(_: i32, y: i32) {
    println!("foo ignore x, y: {y}");
}

enum Msg {
    Hello { id: i32 },
}

enum MsgBool {
    Hello { cond: bool },
}
