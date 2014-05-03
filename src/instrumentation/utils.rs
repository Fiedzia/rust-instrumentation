pub fn dotsplit(s:~str) -> (~str, Option<~str>) {
    //! Split string s into two parts, separated by first . character.
    //! This functions assumes that s is not empty
    //! ie. ~"foo.bar" -> ~"foo", Some(~"bar")
    //! ~"foo" -> ~"foo", None
    match s.find_str(&".") {
        None => (s, None),
        Some(idx) => (s.slice(0, idx).to_owned(), Some(s.slice(idx+1, s.len()).to_owned()))
    }
}
