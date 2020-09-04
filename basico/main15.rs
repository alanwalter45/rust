enum Names{
    Alan,
    Walter,
    Lucas,
    Maria,
    Lucia,
}

fn main(){
    use Names::*;
    let n = Lucas;
    match n {
        Alan=>println!("@lan"),
        Walter=>println!("W@alter"),
        Lucas=>println!("Luc@s"),
        Maria=>println!("Mari@"),
        Lucia=>println!("Luci@"),
    };
    println!("{}",Walter as u8);
}