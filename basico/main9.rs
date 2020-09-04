fn main(){
    let r#mut = "hello";
    println!("{}",r#mut);
    println!("{}",r#if());
    println!("{:x}",'居' as u32);
    println!("{:#^30}","5");
}

fn r#if()->u8{
    10
}