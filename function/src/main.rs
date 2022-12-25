fn main() {
    let my_name = "yavuz";
    let my_surname = "yagis";

    let a = &my_name;
    let b = &my_surname;

    println!("My name is {} {}", *a, *b);
}
