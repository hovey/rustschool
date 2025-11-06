fn append_exclaimation(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut original ="Hello, world".to_string();
    append_exclaimation(&mut original);
    println!("{}", original);
}
