#[inline]
pub fn convert_str_to_underscore_case(s: &str) -> String {
    let mut l: char = '-';
    s.chars()
        .enumerate()
        .fold(String::new(), |mut acc, (i, c)| {
            if i > 0 && c.is_uppercase() && !l.is_whitespace() {
                // (c.is_uppercase() || c.is_whitespace()) {
                acc.push('_');
            }
            l = c;
            acc.push(c);
            acc
        })
        .to_lowercase()
}

#[inline]
pub fn convert_str_to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            word.chars()
                .enumerate()
                .fold(String::new(), |mut acc, (i, c)| {
                    if i == 0 {
                        acc.push_str(&c.to_uppercase().to_string());
                    } else {
                        acc.push_str(&c.to_lowercase().to_string());
                    }
                    acc
                })
        })
        .collect::<Vec<_>>()
        .join(" ")
}
