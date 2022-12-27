use std::io;
fn main() {
    println!("Enter a number to create N:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: i32 = input.trim().parse().expect("Please enter a number only!!");

    if input > 3{
        for i in 0..input{
            for k in 0..input{
                if k==0 || k==input-1 || k==i{
                    print!(" X ");
                }else{
                    print!(" O "); 
                }
            }
            println!("");
        }
    }else{
        println!("please enter the number more than 3");
    }
}