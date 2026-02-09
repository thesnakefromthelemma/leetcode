use std::cmp::min;

pub fn minimum_deletions(s: &str) -> i32 {
    let bytes = s.as_bytes();

    let mut del_pre = 0;
    let mut del_pos = 0;

    for i in 0..bytes.len() {
        /* Unfortunately not optimized into branchlessness by LC?
        match bytes[i] as char {
            'a' => del_pos += 1,
            'b' => del_pre += 1,
            _ => panic!("Unexpected byte in input: {}", bytes[i]),
        }; */
        del_pre += (bytes[i] == b'b') as i32;
        del_pos += (bytes[i] == b'a') as i32;
        del_pos = min(del_pos, del_pre);
    };

    del_pos
}

#[cfg(test)]
mod tests {
    use super::minimum_deletions;

    #[test]
    fn test1() {
        assert_eq!(minimum_deletions("aababbab"), 2)
    }

    #[test]
    fn test2() {
        assert_eq!(minimum_deletions("bbaaaaabb"), 2)
    }
}