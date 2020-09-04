fn main(){
    let numero = 100;
    println!("{}",numero as u8 as char);
    let other_number: u8 = 100;
    println!("{}", other_number as char);
    println!("characters: {} bytes: {}","ჴ".len(),"ჴ".chars().count());
}