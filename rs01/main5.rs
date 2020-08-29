fn main(){
    let num = 5 << 1;
    let num2 = (num as f64)/2.0;
    let num3 = num2 as i32;
    let respuesta = if num3 % 2 ==0{
        String::from("...")
    }else{
        format!("{}",num3)
    };
    println!(">> {}",respuesta);
}