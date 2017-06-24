use std::vec::Vec;
use std::collections::HashMap;
use url::form_urlencoded;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::io;
use std::io::Write;

/// Returns the string at the index of the vec, or None if the index is invalid
#[allow(dead_code)]
pub fn string_at_index<'a>(strings: &'a Vec<String>, index: usize) -> Option<String> {
    if index >= strings.len() {
        return None;
    }
    Some(strings[index].clone())
}

/// Encodes an `HashMap<&str, &str>` to a string which contains the values as urlencoded
/// # Example
///
/// ```rust
/// let v: HashMap<&str, &str> = [
///     ("param", "data"),
///     ("qubit_dfu", "meta")
/// ].iter().cloned().collect();
///
/// // param=data&qubit_dfu=meta
/// let lf = params_to_www_form_url_encoded(&v);
///
/// ```
#[allow(dead_code)]
pub fn params_to_www_form_url_encoded<'a>(params: &'a HashMap<&str,&str>) -> String {
    let mut encoder = form_urlencoded::Serializer::new(String::new());
    for (name, val) in params {
        encoder.append_pair(name, val);
    }
    encoder.finish()
}

#[allow(dead_code)]
pub fn get_md5(clear: &str) -> String {
    let mut hasher = Md5::new();
    hasher.input_str(clear);
    hasher.result_str()
}

#[allow(dead_code)]
pub fn write_flush_stdout<S>(inp: S) where S: Into<String> {
    print!("{}", inp.into());
    flush_stdout();
}

#[allow(dead_code)]
pub fn flush_stdout() {
    io::stdout().flush().ok().expect("Could not flush stdout");
}

#[allow(dead_code)]
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn string_at_index_test() {
        let xvc = vec![String::from("A"), String::from("B"), String::from("C"), String::from("D")];
        assert_eq!(string_at_index(&xvc, 0), Some(String::from("A")));
        assert_eq!(string_at_index(&xvc, 2), Some(String::from("C")));
        assert_eq!(string_at_index(&xvc, 3), Some(String::from("D")));
        assert_eq!(string_at_index(&xvc, 4), None);
    }

    #[test]
    fn params_to_www_form_url_encoded_test() {
        // Empty Test
        let v = HashMap::new();
        assert_eq!(params_to_www_form_url_encoded(&v), "");
        // Create DummyArray
        let v: HashMap<&str, &str> = [
            ("param", "data"),
            ("qubit_dfu", "meta")
        ].iter().cloned().collect();

        // Create & split testing vectors
        let lf = params_to_www_form_url_encoded(&v);
        let rf = String::from("param=data&qubit_dfu=meta");
        let left: Vec<&str> = lf.split('&').collect();
        let right: Vec<&str> = rf.split('&').collect();

        // Compare Vectors
        for t in right {
            if !left.contains(&t) {
                assert!(false, format!("Missing {}", t))
            }
        }
    }

    #[test]
    fn get_md5_test() {
        assert_eq!(get_md5("Hello"), String::from("8b1a9953c4611296a827abf8c47804d7"));
        assert_eq!(get_md5("Meta Charset UTF8"), String::from("6b1b7bab02ef1a5e6370d3950a0f296f"))
    }
}
