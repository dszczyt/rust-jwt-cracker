use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub struct AlphabetGenerator<'a> {
    limit: Option<usize>,
    alphabet: Chars<'a>,
    iterators: Vec<Peekable<Chars<'a>>>,
}

impl<'a> AlphabetGenerator<'a> {
    pub fn init(chars: Chars<'a>) -> Self {
        Self {
            limit: None,
            iterators: vec![],
            alphabet: chars,
        }
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    fn build(&mut self) -> String {
        self.iterators
            .iter_mut()
            .filter_map(|chars| chars.peek())
            .collect()
    }
}

impl<'a> Iterator for AlphabetGenerator<'a> {
    type Item = String;

    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        if self.alphabet.clone().nth(0).is_none() {
            return None;
        }

        for iterator in self.iterators.iter_mut().rev() {
            let next = iterator.next();
            match next {
                Some(_) => {
                    return Some(self.build());
                }
                None => *iterator = self.alphabet.clone().peekable(),
            }
        }

        match self.limit {
            Some(limit) if limit == self.iterators.len() => {
                return None;
            }
            _ => {}
        }

        self.iterators.insert(0, self.alphabet.clone().peekable());

        Some(self.build())
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
