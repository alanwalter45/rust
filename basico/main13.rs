fn main(){
    let name :String = String::from("asa");
    imprimir_nombre(&name);
    imprimir_nombre(&name);
}

fn imprimir_nombre(name:&String)->String{
    println!("{}",name);
}
