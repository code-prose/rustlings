use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
// this shit does NOT work... it expects a tuple struct or tuple variant


fn Ok(x: i8) {
    println!("{}", x);
}

fn Err(x: i8) {
    println!("{}", x);
}
