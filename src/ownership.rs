pub fn ownership() {
    let mut uma_string = String::from("Yuri");
    rouba(&mut uma_string);
    println!("{}", uma_string);
}

fn rouba(string: &mut String) {
    string.push_str(" Grillo");
    println!("{}", string);
}
