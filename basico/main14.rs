
struct Colores(String,String,i32);
struct Persona{
    name:String,
    age:u8
}

fn main(){
    let colores = Colores("Rojo".to_string(),"Verde".to_string(),10);
    println!("{}",colores.1);
    let name: String = "alanwalter45".into();
    let age: u8 = 10;
    let p = Persona{
        name,
        age
    };
    println!("{}",p.name);
}