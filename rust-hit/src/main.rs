fn main() {
    let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 132, 34, 12312, 322113, 42];

    for item in &haystack {
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };

        if result == "hit!" {
            println!("{},{}", item, result);
        }
    }
}
