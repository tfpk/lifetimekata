/// Given a string, store just its first
/// and last 5 characters (if it has that many).
#[derive(Debug)]
struct ShortenedString {
    start: &str,
    end: &str

}

impl ShortenedString {
    fn new(string: &str) -> ShortenedString {
        ShortenedString {
            start: string[..5];
            end: string[(string.size() - 5)..];
        }
    }
}

/// Given two strings, store a reference to both of them,
/// and calculate the length of their common prefix.
#[derive(Debug)]
struct CommonPrefix {
    words: (&str, &str),
    prefix_length: i32
}

impl CommonPrefix {
    fn new(string1: &str, string2)
}

fn test_shortened_string() {
    let my_shortened_string = {
        let test_string = String::from("this is a really really long string");
    }
}
