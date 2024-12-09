pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    let l1 = s1.trim().chars().count();
    let l2 = s2.trim().chars().count();
    if l1 == l2 {
        return None;
    } else if l1 > l2 {
        return Some(&s1);
    } else {
        return Some(&s2);
    }
}

fn main() {
    let s1 = "Hello";
    let s2 = "Hi";
    let s3 = "Yoooouh";
    let s4 = "你好";
    let s5 = "world";
    let output1 = match longer_wish(s1, s2) {
        Some(val) => val,
        None => "Error 1",
    };
    let output2 = match longer_wish(s1, s3) {
        Some(val) => val,
        None => "Error 2",
    };
    let output3 = match longer_wish(s4, s5) {
        Some(val) => val,
        None => "Error 3",
    };
    assert_eq!(output1, "Hello");
    assert_eq!(output2, "Yoooouh");
    assert_eq!(output3, "world");
}
