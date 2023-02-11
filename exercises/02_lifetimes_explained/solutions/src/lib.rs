use require_lifetimes::require_lifetimes;

/// This function returns the reference given to it.
///
/// Make sure it passes this test:
///
/// ```rust
/// use soln02::identity;
///
/// let x = 3;
/// assert_eq!(identity(&x), &x);
/// ````
#[require_lifetimes(!)]
pub fn identity<'a>(number: &'a i32) -> &'a i32 {
    number
}

/// Recall that this function returns `Some(&number)` if
/// number is greater than the other number; otherwise `None`.
///
/// In this case, we know that if the option is `Some`, it will
/// always contain a reference to `number`.
///
/// The following test case will likely always work:
/// ```rust
/// use soln02::only_if_greater;
/// let num = 10;
/// {
///     let greater_than = 7;
///     assert_eq!(Some(&num), only_if_greater(&num, &greater_than))
/// }
/// ````
///
/// But this test case will only work if you've written the
/// lifetimes correctly:
///
/// ```rust
/// use soln02::only_if_greater;
/// let num = 4;
/// let answer = {
///     let greater_than = 7;
///     only_if_greater(&num, &greater_than)
///     // greater_than is dropped here.
/// };
/// assert_eq!(None, answer);
/// ````
#[require_lifetimes(!)]
pub fn only_if_greater<'a, 'b>(number: &'a i32, greater_than: &'b i32) -> Option<&'a i32> {
    if number > greater_than {
        Some(number)
    } else {
        None
    }
}

/// In this case, we know that if the option is `Some`, it will
/// always contain a reference to `number`.
///
/// Recall that this function returns a vector of slices of
/// `text`, split by `delimiter`.
///
/// In this case, we know that the vector will only ever
/// reference `text`, never `delimiter`.
///
/// This example will always work:
///
/// ```rust
/// use soln02::split;
/// let text = String::from("this is a test");
/// let delimiter = String::from(" ");
/// let splitted = split(&text, &delimiter);
/// assert_eq!(splitted, vec!["this", "is", "a", "test"]);
/// ```
///
/// But this example will only work if the lifetimes are correct:
///
/// ```rust
/// use soln02::split;
/// let text = String::from("this is a test");
/// let splitted = {
///     let delimiter = String::from(" ");
///     split(&text, &delimiter)
///     // delimiter is dropped here.
/// };
/// assert_eq!(splitted, vec!["this", "is", "a", "test"]);
/// ```
#[require_lifetimes(!)]
pub fn split<'a, 'b>(text: &'a str, delimiter: &'b str) -> Vec<&'a str> {
    let mut last_split = 0;
    let mut matches: Vec<&str> = vec![];
    for i in 0..text.len() {
        if i < last_split {
            continue;
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

/// Recall that this function returns `&number` if
/// number is greater than `&greater_than`, else it
/// returns `&otherwise`.
///
/// The following test case will likely always work:
///
/// ```rust
/// use soln02::only_if_greater_hard;
/// let num = 4;
/// let otherwise = -1;
/// {
///     let greater_than = 1;
///     assert_eq!(&4, only_if_greater_hard(&num, &greater_than, &otherwise));
///     let greater_than = 5;
///     assert_eq!(&-1, only_if_greater_hard(&num, &greater_than, &otherwise));
/// }
/// ```
///
/// But this test case will only work if you've written
/// the lifetimes correctly:
///
/// ```rust
/// use soln02::only_if_greater_hard;
/// let num = 10;
/// let otherwise = -1;
/// let answer = {
///     let greater_than = 7;
///     only_if_greater_hard(&num, &greater_than, &otherwise)
///     // greater_than is dropped here.
/// };
/// assert_eq!(&num, answer);
///
/// let answer = {
///     let greater_than = 100;
///     only_if_greater_hard(&num, &greater_than, &otherwise)
///     // greater_than is dropped here.
/// };
/// assert_eq!(&otherwise, answer);
/// ```
///
/// And this test case should never compile -- can you see why?
///
/// ```rust,compile_fail
/// use soln02::only_if_greater_hard;
/// let greater_than = 7;
/// let otherwise = -1;
/// let answer = {
///     let num = 10;
///     only_if_greater_hard(&num, &greater_than, &otherwise)
///     // num is dropped here
/// };
/// assert_eq!(&num, answer);
/// ```
#[require_lifetimes(!)]
pub fn only_if_greater_hard<'a, 'b>(
    number: &'a i32,
    greater_than: &'b i32,
    otherwise: &'a i32,
) -> &'a i32 {
    if number > greater_than {
        number
    } else {
        otherwise
    }
}
