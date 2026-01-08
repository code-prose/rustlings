fn main() {
    let mut r;

    {
        let x = 5;
        r = &x;
        println!("r: {r}");
    }

    let y = 4;
    r = &y;
    println!("r: {r}");
}
