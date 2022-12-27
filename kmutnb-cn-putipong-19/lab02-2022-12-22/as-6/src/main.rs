fn main() {
    let mut input = [1,2,0,4,3,0,5,0];
    print!("input: arr[] = ");
    for j in input{
        print!("{} ",j);
    }

    for i in 0..input.len(){
        for k in i..input.len(){
            if input[i] == 0{
                input[i] = input[k];
                input[k] = 0;
            }
        }
    }
    println!("");
    print!("output: arr[] = ");
    for i in input{
        print!("{} ",i);
    }

}