// fn starts_with_a<'a, 'b>(s1: &'a str, s2: &'a str) -> &'a str {
fn starts_with_a<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.starts_with('a') {
        return s1;
    }

    if s2.starts_with('a') {
        return s2;
    }

    // If neither starts with 'a', return the first slice
    s1

}


fn main() {
    let string1 = "example";
    let string2 = "sample";
    let _result = starts_with_a(string1, string2);
}
