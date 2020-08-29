fn main(){
    let phone = Phones::Samsung;
    search(phone);
    let phone = Phones::Nokia{model: String::from("1100")};
    search(phone);
    let phone = Phones::IPhone;
    search(phone);
}

fn search(phone:Phones){
    match phone {
        Phones::IPhone => println!("Es un Iphone"),
        Phones::Nokia{model} => println!("Es un Nokia {}",model),
        Phones::Samsung => println!("Es un Samsung")
    }
}

enum Phones{
    Samsung,
    Nokia{model: String},
    IPhone
}