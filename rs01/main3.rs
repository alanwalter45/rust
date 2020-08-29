fn main(){
    let v1 : [i32;3] = [1,2,3];
    println!("tamanho:{:?}",v1.len());
    println!("tupla {:?}",convert_to_tuple(&v1));
}

fn convert_to_tuple(vector:&[i32])->(i32,i32,i32){
    (vector[0],vector[1],vector[2])
}