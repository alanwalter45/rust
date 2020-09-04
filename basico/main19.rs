use std::process::Command;

fn main(){
    let output = Command::new("ls")
                        .args(&["-la"])
                        .output()
                        .expect(r#"No se logro invocar el comando "ls""#);
    println!("{}", String::from_utf8_lossy(&output.stdout).to_uppercase());
}