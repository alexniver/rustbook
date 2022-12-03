fn main() {
    let mut num = 7;
    // let num = 3;
    if num < 5 {
        println!("num is less than 5");
    } else {
        println!("num is equal bigger than 5");
    }

    if num == 7 {
        println!("num is seven");
    }

    num = 6;

    if num % 4 == 0 {
        println!("num divisible by 4");
    } else if num % 3 == 0 {
        println!("num divisible by 3");
    } else if num % 2 == 0 {
        println!("num divisible by 2");
    }

    let cond = false;
    num = if cond { 22 } else { 33 };

    println!("cond: {cond}, num : {num}");

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("counter: {counter}, result: {result}");

    counter = 0;

    'sign_outer: loop {
        loop {
            if counter == 9 {
                println!("get nine, {counter}");
                break 'sign_outer;
            } else {
                println!("not get nine");
                break;
            }
        }
        counter += 1;
    }
}
