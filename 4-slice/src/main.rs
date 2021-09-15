fn main() {
    let mut s = String::from("hello world");
    let s1 = first_word(&s);
    string_slice(&s);
    let s2 = first_word_slice(&s);
    println!("s1=={}", s1);
    println!("s2=={}", s2);
    s.clear();
    println!("{}", s1);
    other_slice();
}
// 找到第一个空格
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
// slice
fn string_slice(s: &String) {
    let hello = &s[0..5];
    println!("{}", hello);
    let world = &s[6..11];
    println!("{}", world);
    let len = s.len();
    let s1 = &s[3..len];
    println!("{}", s1);
    // &s[0..len] 等同 &s[..]
    let s2 = &s[0..len];
    let s3 = &s[..];
    println!("{}", s2 == s3)
}
// 改良款slice
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn other_slice() {
    let a = [1, 2, 3, 4, 5];
    let a1 = &a[0..3];
    println!("{:?}", a1);
}
