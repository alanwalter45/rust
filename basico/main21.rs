fn encontrar_valor(num:u32)->Option<u32>{
    if num<10{
        None
    }else{
        Some(num)
    }
}

fn main(){
    let r = encontrar_valor(20);
    if r.is_some(){
        println!("{}",r.unwrap());
    }
}