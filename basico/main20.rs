#[derive(Debug)]
enum Tipo{
    A,
    B,
}

struct Editor{
    name: String,
    tipo : Tipo,
}

impl Editor{
    fn new()->Self{
        Self{
            name:String::from("aw45"),
            tipo:Tipo::A,
        }
    }
    fn edit_name(&mut self){
        self.name = String::from("alanwalter45");
        self.tipo = Tipo::B;
    }
    fn get_info(&self)->String{
        format!("name: {} tipo: {:?}",self.name,self.tipo)
    }
}

fn main(){
    let mut editor = Editor::new();
    let resultado = editor.get_info();
    println!("{}",resultado);
    editor.edit_name();
    let resultado = editor.get_info();
    println!("{}",resultado);
}