fn main(){
    let mut list = Vec::new();
    for i in 0..=10 {
        //println!("{}",i+1);
        list.push(i+1);
    }
    println!("{:?}",list);
}