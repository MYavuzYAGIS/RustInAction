fn trial() {
    let needle = 12;
    let haystack = [1, 2, 3, 4, 5, 312, 4232, 12212, 4232, 1231];
    for item in &haystack {
        if *item == needle {
            println!("{}", item);
        } else {
            println!("{} not found", item);
        }
    }
}

fn main() {
    let my_name = "yavuz";
    let my_surname = "yagis";

    let a = &my_name;
    let b = &my_surname;

    println!("My name is {} {}", *a, *b);
    trial();
}
