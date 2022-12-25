fn main() {
    let a = 42;
    let r = &a; // & == su noktaya referans et, sadece memory address olarak calisiyor.
    let b = a + *r; // su noktadaki adresin valuesini al.

    // & ==> reelden adrese * ==> adresten value'ye.

    println!("a + a = {}", b)
}
