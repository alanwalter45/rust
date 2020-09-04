fn main(){
    let n = "email".to_string();
    let name: String = "alanwalter45".into();
    const X :u8 = 10;
    println!("{n} <{name}> , {x}",name=name,n=n,x=X);
    let number = 8;
    let number2 = & number;
    println!("{}",number2+3);
}