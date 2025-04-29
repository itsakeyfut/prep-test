fn main() {
    // スタック上のデータ
    let x = 5;
    let y = [1, 2, 3, 4, 5]; // 固定長配列もスタック

    // ヒープ上のデータ
    let v = vec![1, 2, 3, 4, 5]; // Vecはヒープ
    let s = String::from("Hello"); // Stringもヒープ

    println!("x: {}", x);
    println!("y: {:?}", y);
    println!("v: {:?}", v);
    println!("s: {}", s);
}
