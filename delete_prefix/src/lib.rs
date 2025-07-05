pub fn delete_prefix<'a, 'b>(prefix: &'b str, s: &'a str) -> Option<&'a str> {
    Some(s.strip_prefix(prefix)?)
}
