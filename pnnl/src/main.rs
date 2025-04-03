fn main() {
    println!("Hello, world!");
    let x = 10;
    println!("{x}");

    let str1 = String::from("hello");  // owned structure

    // gets built into the binary as a static reference, str2 is pointer to the value
    let str2 = "world";

    let str3 = str1;
    // try print str1
    println!("{str1}");
}
