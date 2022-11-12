fn get_time(s: &str) -> usize {
    let alphabet = [
        "A", "B", "C", "D", "E", "F", "G", "J", "I", "H", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"
        ];
    let mut result = 0;
    let mut cur = 0usize;

    for c in s.chars().into_iter() {
        let index_in_alphabet = alphabet.iter().position(
            |&r| r.chars().next().unwrap() == c
        ).unwrap();
        let gap  ;
        if cur >  index_in_alphabet {
            gap = cur - index_in_alphabet;
        } else {
            gap = index_in_alphabet - cur;
        }
        cur = index_in_alphabet;
        if gap > alphabet.len() / 2 {
            result = result + (alphabet.len() - gap);
        } else {
           result = result + gap;
        }
    }   
    result
}

#[cfg(test)]
mod test {
    use super::get_time;


    #[test]
    fn basic() {
        let s = "BZA";

        let result = get_time(s);

        assert_eq!(result, 4);
    }
    #[test]
    fn complicate() {
        let s = "ZNMD";

        let result = get_time(s);

        assert_eq!(result, 23);
    }
}