fn main(){
    let no_mut_string = String::from("Hello Rust!");
    println!("String is {}", no_mut_string);
    let mut mutable_string = String::from("Hello C");
    println!("String before: {}", mutable_string);
    mutable_string.push_str(", C is fantastic but Rust is better");
    println!("String after: {}", mutable_string);
    mutable_string = String::from("Hello Rust");
    println!("String re-assigned: {}", mutable_string);
    let mut str_copy = mutable_string.clone();
    println!("String copy: {}", str_copy);
    println!("{}, has {} char length", str_copy , str_copy.len());
    str_copy.clear();
    println!("str_copy is: {}", str_copy);
    // or with pointer
    let mut str_copy = &no_mut_string;      //re-declare because String struct not accept &string type
    println!("Now str_copy is : {}", str_copy);
    // try to slice it
    println!("Slice is {}", &str_copy[0..2]);
}