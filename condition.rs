fn main(){
    let x:i8 = 2;
    let number = if x > 0 { 0 } else { 5 };
    let mut i:i8 = 10;
    if x > 0{
        println!("{} is greater than 0", x);
    }
    loop{
        
        if i > 0{
            println!("Here we go!, repeat: {}", i);
            i -= 1;
        }else{
            break;
        }
    }
    i = 10;
    let tab10:i8 = loop{
          
        if i > 0{
            println!("{} x {} = {}", 10 , i , 10*i);
            i -= 1;
        }else{
            break 10*i;
        } 
    };
    println!("Last operation is {}", tab10);
    i = 2;
    while i > 0{
        println!("Hi from while loop!");
        i -= 1;
    }
    let for_array:[i8; 5] = [1,2,3,4,5];
    for num in for_array{
        println!("The number from array is {}", num);
    }
    for num in (1..5).rev(){
        println!("Number is {}", num);
    }
}