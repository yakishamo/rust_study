use std::io;

fn main() {
    println!("Enter degree Fahrenheit");
    
    let mut F = String::new();
    io::stdin()
        .read_line(&mut F)
        .expect("Failed to read line");

    let F: f32 = F.trim().parse()
        .expect("Enter integer");

    let C: f32 = (F - 32.0) / 1.8;
    println!("C : {}", C);
}
