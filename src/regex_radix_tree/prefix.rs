macro_rules! next_char_or_return {
    ($c:expr,$p:expr) => {
        match $c.next() {
            Some(char) => char,
            None => return $p,
        }
    };
}

pub fn common_prefix(left: &str, right: &str) -> String {
    let size = common_prefix_char_size(left, right);

    get_prefix_with_char_size(left, size)
}

pub fn common_prefix_char_size(left: &str, right: &str) -> u32 {
    let mut prefix_length = 0;
    let mut left_chars = left.chars();
    let mut right_chars = right.chars();
    let mut was_escape = false;
    let mut group_level = 0;
    let mut i = 0;

    loop {
        let left_char = next_char_or_return!(left_chars, prefix_length);
        let right_char = next_char_or_return!(right_chars, prefix_length);

        if left_char != right_char {
            return prefix_length;
        }

        if left_char == '(' && !was_escape {
            group_level += 1;
        } else if left_char == ')' && !was_escape {
            group_level -= 1;
        }

        if left_char == '\\' && !was_escape {
            was_escape = true;
        } else if was_escape {
            was_escape = false;
        }

        i += 1;

        if group_level == 0 && !was_escape {
            prefix_length = i;
        }
    }
}

pub fn get_prefix_with_char_size(str: &str, size: u32) -> String {
    if size == 0 {
        return "".to_string();
    }

    let mut chars = str.chars();
    let mut prefix = Vec::new();

    for _i in 0..size {
        match chars.next() {
            Some(char) => prefix.push(char),
            None => {
                return prefix.into_iter().collect();
            }
        }
    }

    prefix.into_iter().collect()
}
