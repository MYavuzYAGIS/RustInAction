#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

#[derive(Debug)]
struct Family {
    name: String,
    size: i8,
    household: Vec<u8>,
}

pub fn structs() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let yagis = Family {
        name: String::from("yagis"),
        size: 3,
        household: Vec::new(),
    };

    let family_name = &yagis.name;
    let family_size = &yagis.size;
    let family_household = &yagis.household.len();

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);

    println!(
        "Our family name is {}, we are {} people",
        family_name, family_size
    )
}
