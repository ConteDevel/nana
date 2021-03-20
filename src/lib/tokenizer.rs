fn parse_str(source: &str, start: usize, idx: &mut usize) -> String {
    while let Some(ch) = source.chars().nth(*idx) {
        if !ch.is_alphanumeric() {
            break;
        }
        *idx += 1;
    }
    return source.chars().skip(start).take(*idx - start).collect();
}

pub fn tokenize(source: &str, mut idx: &mut usize) -> Option<String> {
    let mut start = *idx;
    while let Some(ch) = source.chars().nth(*idx) {
        if ch == '\'' || ch == ':' || ch == '=' {
            *idx += 1;
            return Some(String::from(ch));
        } else if ch.is_alphanumeric() {
            return Some(parse_str(&source, start, &mut idx));
        } else {
            *idx += 1;
            start = *idx;
        }
    }
    return None;
}