fn main(){

    let mut v: Vec<i8> = Vec::new();
    v.push(1);
    v.push(2);
    let mut vet_cpy = &v;
    let value = &v[1];
    println!("Print first(0) element is {}", v[0]);
    println!("Print first(0) element of cpy array is {}", vet_cpy[0]);
    println!("Value is {}", value);
    v.push(3);
    v.push(100);
    for i in &v{
        println!("Array is {}", i);
    }
}
