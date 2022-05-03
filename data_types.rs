fn main(){
    let mut int:i8 = 5;
    int += 10;
    const PICONST:f32 = 3.14;   //or f64 if double-precision ieee-754
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //let (x, y, z) = tup;
    //or
    let v3:u8 = tup.2;
    let numarray = [1,2,3,4];
    let stringarray = ["I", "Love", "Torvalds"];
    let fixedarray: [i8; 2] = [5,10];
    let index:usize = 1;
    println!("Int : {}", int);
    println!("Pi : {}", PICONST);
    println!("2 element of tuple: {}", v3);
    println!("{} element of array is: {}", index , fixedarray[index]);
}