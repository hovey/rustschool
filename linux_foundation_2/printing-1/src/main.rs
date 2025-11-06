fn main() {
    let a: i64 = 42;
    let s = "Hello Rustaceans";
    let h: u32 = 4_207_849_484;
    let fname = "Larry";
    let lname = "Wall";

    println!();
    println!("a == {}", a);
    println!("s == {}", s);

    println!("x == {:X}", h);
    println!("x == {:x}", h);

    println!("{0} {1}", fname, lname);
    println!("{fname} {lname}");
}
