use std::{convert::TryInto, str::Chars};

#[derive(Debug)]
pub struct AlphabetGenerator<'a> {
    alphabet: Chars<'a>,
    alphabet_size: usize,
    value: usize,
    limit: Option<usize>,
    current_length: usize,
}

impl<'a> AlphabetGenerator<'a> {
    pub fn init(chars: Chars<'a>) -> Self {
        Self {
            value: 0,
            alphabet_size: chars.clone().count(),
            alphabet: chars,
            limit: None,
            current_length: 1,
        }
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl<'a> Iterator for AlphabetGenerator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.alphabet_size == 0 {
            return None;
        }
        if let Some(limit) = self.limit {
            if self.current_length > limit {
                return None;
            }
        }
        let mut result = "".to_string();
        let mut tmp: usize = self.value;
        loop {
            let div = tmp / self.alphabet_size;
            let rem = tmp - div * self.alphabet_size;
            result.insert(0, self.alphabet.clone().nth(rem).unwrap());
            tmp = div;
            if tmp == 0 {
                break;
            }
        }
        result.insert_str(
            0,
            &str::repeat(
                &self.alphabet.clone().nth(0).unwrap().to_string(),
                self.current_length - result.len(),
            ),
        );
        self.value += 1;
        if self.value
            == self
                .alphabet_size
                .pow(self.current_length.try_into().unwrap())
        {
            self.value = 0;
            self.current_length += 1;
        }
        return Some(result);
    }
}

#[cfg(test)]
mod tests {
    use super::AlphabetGenerator;

    #[test]
    fn empty_alphabet() {
        let mut gen = AlphabetGenerator::init("".chars());
        let next = gen.next();
        assert_eq!(next, None);
    }

    #[test]
    fn limit_zero() {
        let mut gen = AlphabetGenerator::init("a".chars()).with_limit(0);
        let next = gen.next();
        assert_eq!(next, None);
    }

    #[test]
    fn single_char_alphabet() {
        let mut gen = AlphabetGenerator::init("a".chars());
        let next = gen.next();
        assert_eq!(next, Some("a".to_string()));
        let next = gen.next();
        assert_eq!(next, Some("aa".to_string()));
        let next = gen.next();
        assert_eq!(next, Some("aaa".to_string()));
        let next = gen.next();
        assert_eq!(next, Some("aaaa".to_string()));
    }

    #[test]
    fn single_char_alphabet_with_limit() {
        let mut gen = AlphabetGenerator::init("a".chars()).with_limit(3);
        let next = gen.next();
        assert_eq!(next, Some("a".to_string()));
        let next = gen.next();
        assert_eq!(next, Some("aa".to_string()));
        let next = gen.next();
        assert_eq!(next, Some("aaa".to_string()));
        let next = gen.next();
        assert_eq!(next, None);
    }

    #[test]
    fn two_chars_alphabet() {
        let mut gen = AlphabetGenerator::init("ab".chars());
        let next = gen.next();
        assert_eq!(next, Some("a".to_string()));
        let next = gen.next();
        assert_eq!(next, Some("b".to_string()));
        let next = gen.next();
        assert_eq!(next, Some("aa".to_string()));
        let next = gen.next();
        assert_eq!(next, Some("ab".to_string()));
        let next = gen.next();
        assert_eq!(next, Some("ba".to_string()));
        let next = gen.next();
        assert_eq!(next, Some("bb".to_string()));
        let next = gen.next();
        assert_eq!(next, Some("aaa".to_string()));
        let next = gen.next();
        assert_eq!(next, Some("aab".to_string()));
    }
}
