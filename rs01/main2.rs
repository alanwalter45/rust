#[derive(Debug)]
struct Data(u16);

#[derive(Debug)]
struct Persona<'a>{
    name:&'a str,
    age:u8
}

fn main() {  
    println!("data:{:?}",Data(5));
    let name = "Alan";
    let age = 15;
    let alan = Persona{name,age};
    println!("{:#?}",alan);
}
