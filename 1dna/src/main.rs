use std::io::Read;

fn read_next_byte() {
    return std::io::stdin()
        .bytes() 
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as i32);
}

fn main() {
    

    println!("{:?}", input);
}
