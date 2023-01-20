fn main() {
    let num:usize  = 100;
    let ans:usize = sumary(num);
    println!("1 + 2 + 3 + ... + 100  =  {}",ans);

}
fn sumary(x:usize) ->usize {
    let mut sum:usize =0;
    for i in 1..x+1{
        sum += i;
    }
    return sum;
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn sumary() {
        assert_eq!(sumary(100), 5050);
   }

}
