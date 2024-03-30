fn main() {
    let some_number: Option<i32> = Option::Some(3);
    let some_char: Option<char> = Some('e');
    // Required to annotate here:
    let absent_num: Option<i32> = None;
}
