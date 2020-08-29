fn main(){
    let (v1,v2,v3) = (3,2,1);
    println!("tamanho:{}{}{}",v1,v2,v3);
    let t = (3,2,1);
    println!("tamanho:{:?}",t);
    let t:(i32,i32,i32) = (3,2,1);
    println!("tamanho:{:?}",t);
}