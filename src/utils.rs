use std::str::Chars;

// TODO: map al space chars
const SPACE_CHARS: &'static [char] = &[' ', '\t', '\r', '\n'];

pub struct SplitWords<'a> {
    chars: Chars<'a>,
    buffer: Option<char>,
}

impl<'a> SplitWords<'a> {
    pub fn new<'b>(s: &'b str) -> SplitWords<'b> {
        SplitWords {
            chars: s.chars(),
            buffer: None,
        }
    }
}

impl<'a> Iterator for SplitWords<'a> {
    type Item = String;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some(buffer) = self.buffer {
            self.buffer = None;
            return Some(buffer.to_string());
        }

        let mut res = String::new();
        loop {
            match self.chars.next() {
                Some(buffer) => {
                    if SPACE_CHARS.contains(&buffer) {
                        if res.is_empty() {
                            return Some(buffer.to_string());
                        } else {
                            self.buffer = Some(buffer);

                            return Some(res);
                        }
                    } else {
                        res.push(buffer);
                    }
                }
                None => {
                    if res.is_empty() {
                        return None;
                    } else {
                        return Some(res);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SplitWords;

    #[test]
    fn split_words_iterator() {
        let known: Vec<String> = [
            "Ciao", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "Ciao,", " ", "ciao",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();
        let value: Vec<String> = SplitWords::new("Ciao          Ciao, ciao").collect();
        assert_eq!(known, value);
    }
}
