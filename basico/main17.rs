fn metodo_generico<T>(a:T)->T{
    a
}
fn main(){
    let a = metodo_generico(5);
    println!("{}",a);
    let a = metodo_generico("Hello");
    println!("{}",a);
    let a = metodo_generico(vec![1,2,3]);
    println!("{:?}",a);
}