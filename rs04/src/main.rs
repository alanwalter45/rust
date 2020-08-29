use std::fmt;
use std::io;

fn main() {
    let mut name = String::new();
    println!("Escriba su nombre: ");
    io::stdin()
        .read_line(&mut name)
        .expect("no se logro leer el nombre");
    let user = User {
        name: name,
        email: String::from("hi"),
        age: 15,
    };
    println!("{}", user);
}

struct User {
    name: String,
    email: String,
    age: i32,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.name, self.email, self.age)
    }
}
