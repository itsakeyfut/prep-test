fn print_head(n: u32) {
    if n > 0 {
        print_head(n - 1);
        println!("{}", n);
    }
}

fn main() {
    let n = 5;
    print_head(n);

    /*
     * 1
     * 2
     * 3
     * 4
     * 5
     */
}
