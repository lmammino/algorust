pub fn findmatch(needle: String, haystack: String) -> Option<usize> {
    let n = needle.len();
    let h = haystack.len();

    if n <= h {
        for i in 0..=h.wrapping_sub(n) {
            let mut j = 0;
            while j < n && haystack[i + j..=i + j] == needle[j..=j] {
                j = j + 1;
            }
            if j == n {
                return Some(i);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn finds_substring() {
        let i = super::findmatch("abba".to_owned(), "aababbab".to_owned());
        assert_eq!(i, Some(3));
    }

    #[test]
    fn finds_prefix() {
        let i = super::findmatch("abba".to_owned(), "abbabba".to_owned());
        assert_eq!(i, Some(0));
    }

    #[test]
    fn finds_suffix() {
        let i = super::findmatch("abba".to_owned(), "aababba".to_owned());
        assert_eq!(i, Some(3));
    }

    #[test]
    fn returns_none_if_not_found() {
        let i = super::findmatch("abba".to_owned(), "aababxba".to_owned());
        assert_eq!(i, None);
    }
}
