use std::io;
fn main() {
    println!("Enter a number to find a factor:");

    let mut num = String::new();
    io::stdin().read_line(&mut num)
        .expect("Failed to read line");

    let num: i32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number only!!");
            return;
        },
    };

    
    print!("The factor of {} = ", num);
    let mut n = num;
    let mut first = true;
    for i in 2.. {
        while n % i == 0 {
            if first {
                first = false;
            } else {
                print!("*");
            }
            print!("{}", i);
            n /= i;
        }
        if n == 1 {
            break;
        }
    }
}