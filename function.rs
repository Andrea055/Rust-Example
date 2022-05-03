fn main(){
let x:i8 = 5;
let y:i8 = 5;
println!("The result of {} + {} is {}", x , y, sum(x,y));
}

fn sum(x: i8 , y: i8) -> i8{
    x + y
}