use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let numero_generado = rand::thread_rng().gen_range(1, 100);
    //println!("numero generado: {}", numero_generado);
    loop {
        println!("introduce un numero:");
        let mut mi_numero = String::new();
        io::stdin()
            .read_line(&mut mi_numero)
            .expect("Fallo al leer la linea");
        //let mi_numero : u32 = mi_numero.trim().parse().expect("No se logro convertir");
        let mi_numero: u32 = match mi_numero.trim().parse() {
            Ok(numero) => numero,
            Err(_) => continue,
        };
        match mi_numero.cmp(&numero_generado) {
            Ordering::Greater => println!("el numero {} es mayor", mi_numero),
            Ordering::Less => println!("el numero {} es menor", mi_numero),
            Ordering::Equal => {
                println!("ganaste");
                break;
            }
        }
    }
}
