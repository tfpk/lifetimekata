// In this case, we know that if the option is `Some`, it will
// contain a reference to `number`.
fn only_if_greater<'a>(number: &'a i32, greater_than: &'a i32) -> Option<&'a i32> {
    if number > greater_than {
        Some(number)
    } else {
        None
    }
}

fn split<'a>(text: &'a str, delimiter: &'a str) -> Vec<&'a str> {
    let mut last_split = 0;
    let mut matches: Vec<&str> = vec![];
    for i in 0..text.len() {
        if i < last_split {
            continue
        }
        if text[i..].starts_with(delimiter) {
            matches.push(&text[last_split..i]);
            last_split = i + delimiter.len();
        }
    }
    if last_split < text.len() {
        matches.push(&text[last_split..]);
    }

    matches
}

fn only_if_greater_hard<'a>(number: &'a i32, greater_than: &'a i32, otherwise: &'a i32) -> &'a i32 {
    if number > greater_than {
        number
    } else {
        otherwise
    }

}

fn main() {
    // This should work fine no matter what the lifetimes are.
    let num = 4;
    {
        let greater_than = 7;
        assert_eq!(Some(&num), only_if_greater(&num, &greater_than))
    }

    // This may not work correctly if the lifetimes are incorrect.
    let num = 10;
    let answer = {
        let greater_than = 7;
        only_if_greater(&num, &greater_than)
    };
    assert_eq!(Some(&num), answer);

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_only_if_greater() {
    }
}
