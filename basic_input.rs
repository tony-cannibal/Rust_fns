


fn main() {
    let s = read_string();    
        
    println!("{}", s);
}

fn read_string() -> String {
    let mut input:String = String::new();
    std::io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    input
}
