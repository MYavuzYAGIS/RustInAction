use regex::Regex;

fn main() {
    let re = Regex::new("picture").unwrap();
    let quote = " Every face, every shop, bedroom window, public-house, 
    and dark square is a picture feverishly turned--in search of what?
    It is the same with books. What do we seek through millions of pages?";
}

// fn main() {
//     let search_term = "picture";
//
//     for (i, line) in quote.lines().enumerate() {
//         if line.contains(search_term) {
//             let line_num = i + 1;
//             println!("{},{}", line_num, line)
//         }
//     }
// }
