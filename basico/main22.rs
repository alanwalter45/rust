fn prints_vec(vector: Vec<i32>) {
    if vector.len() != 2 {
        panic!("deberia tener 2 elementos");
    }
    println!("{}, {}", vector[0], vector[1]);
}

fn main() {
    let my_vec = vec![8, 9];
    prints_vec(my_vec);
}