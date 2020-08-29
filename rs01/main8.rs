fn main(){
    let c = Car{
        name:"Alan",
        num:45
    };
    println!("{}",c.show_data());
}

struct Car<'a,T>{
    name: &'a str,
    num : T,
}

impl Car<'_,i32>{
    fn show_data(&self)->String{
        format!("{} {}",self.name,self.num)
    }
}