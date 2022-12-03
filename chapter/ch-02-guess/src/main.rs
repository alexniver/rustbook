use rand::Rng;

fn main() {
    println!("Guess the number!");

    let mut secret_num = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess, must be a number, 'exit' for quit");
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("read input error");

        let input = input.trim();
        // if input == "exit" {
        //
        // }
        //
        if input == "exit" {
            println!("bye~");
            break;
        }
        println!("you guess : {input}");
        let input_num = match input.parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match secret_num.cmp(&input_num) {
            std::cmp::Ordering::Less => {
                println!("secret num is less than {input_num}!");
            }
            std::cmp::Ordering::Equal => {
                println!("right!");
                secret_num = rand::thread_rng().gen_range(1..=100);
                println!("secret num changed!");
            }
            std::cmp::Ordering::Greater => {
                println!("secret num is bigger than {input_num}!");
            }
        }

        // if secret_num == input_num {
        //     println!("right!");
        //     secret_num = rand::thread_rng().gen_range(1..=100);
        //     println!("secret num changed!");
        // } else if secret_num > input_num {
        //     println!("secret num is bigger than {input_num}!");
        // } else {
        //     println!("secret num is less than {input_num}!");
        // }
    }
}
