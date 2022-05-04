struct User {
    name: String,
    surname: String,
    birth: String,
    male: bool,
}
struct Rectangle {
    width: f64,
    height: f64,
}
impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
fn main() {
    let user = User {
        name: String::from("Andrea"),
        surname: String::from("Canale"),
        birth: String::from("19/05/2005"),
        male: true,
    };
    println!("My name is {}", user.name);
    let user = User {
        name: String::from("Jon"),
        ..user
    };
    println!("Now name is {}", user.name);
    let user2 = retstruct(
        String::from("Linus"),
        String::from("Torvalds"),
        String::from("28/12/1969"),
        true,
    );
    println!(
        "User2 is the greatest {} {} born {}",
        user2.name, user2.surname, user2.birth
    );
    let ret1 = Rectangle {
        width: 43.2,
        height: 20.2,
    };
    println!("The area of ret1 is {}", ret1.area());
}

//test with a function

fn retstruct(name: String, surname: String, birth: String, male: bool) -> User {
    User {
        name: name,
        surname: surname,
        birth: birth,
        male: male,
    }
}
