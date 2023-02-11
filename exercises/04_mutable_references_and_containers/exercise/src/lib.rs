use require_lifetimes::require_lifetimes;

/// This function takes in a "vector" of `&strs`, a "loc" `usize`
/// and a "new" `&str`. Your job is to replace the old string at the
/// location (i.e., array index) "loc" with the "new" one.  Don't do
/// anything if "loc" is beyond the end of "vector".
///
/// Make sure it passes this test:
///
/// ```rust
/// use ex04::vector_set;
///
///
/// // Create a vector of strings.
/// let strings = vec!["Hello".to_string(), "My".to_string(), "Name".to_string(), "Is".to_string(), "Tom".to_string()];
///
/// // Create some strings to replace inside that vector.
/// let your = "Your".to_string();
/// let unknown = "Unknown".to_string();
///
///
/// // Create a vector of references to the string vector.
/// let mut message: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
///
/// // Set some references
/// vector_set(&mut message, 1, &your);
/// vector_set(&mut message, 4, &unknown);
/// vector_set(&mut message, 10, &unknown);
///
/// // Hopefully, they're now equal
/// assert_eq!(message , vec!["Hello", "Your", "Name", "Is", "Unknown"]);
/// ````
#[require_lifetimes(!)]
pub fn vector_set(vector: &mut Vec<&str>, loc: usize, new: &str) {
    // TODO: You will need to write this code yourself.
    //       Don't worry, it's only one line long.
    todo!()
}
