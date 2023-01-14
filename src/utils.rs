pub fn is_valid_ending_quote(chars: &Vec<char>, idx: usize) -> bool {
    if chars[idx] == '"' {
        if idx == 0 {
            return false;
        }
        let mut is_escaped = false;
        let mut i = idx - 1;
        while chars[i] == '\\' {
            is_escaped = !is_escaped;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        return !is_escaped;
    } else {
        return false;
    }
}

pub fn match_literal(chars: &Vec<char>, idx: usize, literal: &str) -> bool {
    if idx <= chars.len() - literal.len() {
        for (i, c) in literal.chars().enumerate() {
            if chars[idx + i] != c {
                return false;
            }
            return true;
        }
    }
    return false;
}

pub fn skip_spaces(chars: &Vec<char>, idx: &mut usize) {
    skip_chars(chars, idx, &[' ', '\r', '\n']);
}

pub fn skip_chars(chars: &Vec<char>, idx: &mut usize, to_skip: &[char]) {
    while *idx < chars.len() && to_skip.contains(&chars[*idx]) {
        *idx += 1;
    }
}

pub fn skip_util_char(chars: &Vec<char>, idx: &mut usize, util_chars: &[char]) {
    while *idx < chars.len() && !util_chars.contains(&chars[*idx]) {
        *idx += 1;
    }
}

pub fn nearby_content(chars: &Vec<char>, idx: usize) -> String {
    let start = if idx < 50 { 0 } else { idx - 50 };
    chars[start..=idx].iter().collect::<String>()
}

pub fn parse_error(chars: &Vec<char>, idx: usize) -> ! {
    panic!("{}", nearby_content(chars, idx));
}

#[cfg(test)]
mod test {
    use super::is_valid_ending_quote;

    #[test]
    fn valid_ending_quote() {
        let s = r#"aaaaa""#;
        let r = is_valid_ending_quote(&s.chars().collect(), s.len() - 1);
        assert_eq!(r, true);

        let s = r#"aaaa\""#;
        let r = is_valid_ending_quote(&s.chars().collect(), s.len() - 1);
        assert_eq!(r, false);

        let s = r#"aaa\\""#;
        let r = is_valid_ending_quote(&s.chars().collect(), s.len() - 1);
        assert_eq!(r, true);
    }
}