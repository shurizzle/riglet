use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

use encoding::{all::ISO_8859_1, Encoding};
use figfont::FIGfont;

use crate::{line::FIGline, utils::SplitWords};

pub struct FIGure<'a> {
    width: usize,
    font: &'a FIGfont,
    lines: Vec<FIGline<'a>>,
}

impl<'a> FIGure<'a> {
    pub fn new<'b>(font: &'b FIGfont, width: usize) -> FIGure<'b> {
        FIGure {
            width,
            font,
            lines: Vec::new(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn font(&self) -> &'a FIGfont {
        &self.font
    }

    pub fn add_char(&mut self, ch: char) -> Result<(), Cow<str>> {
        self.add(&ch.to_string())
    }

    pub fn add<S: AsRef<str>>(&mut self, text: S) -> Result<(), Cow<str>> {
        let text = text.as_ref();
        let mut words: Vec<FIGline> = Vec::new();
        for word in SplitWords::new(text) {
            let chars = ISO_8859_1.encode(&word, encoding::EncoderTrap::Replace)?;
            let mut line = FIGline::new(&self.font);

            for c in chars {
                let old_line = line.clone();
                line.add_char(c as i32);

                if line.width() > self.width() {
                    words.push(old_line);
                    line = FIGline::new(&self.font);
                }
            }

            if !line.is_empty() {
                words.push(line);
            }
        }

        for word in words {
            if self.lines.is_empty() {
                self.lines.push(word);
            } else {
                let l = self.lines.len();
                let mut line = self.lines.remove(l - 1);
                let old_line = line.clone();
                line.add_line(&word);

                if line.width() > self.width {
                    self.lines.push(old_line);
                    self.lines.push(word);
                } else {
                    self.lines.push(line);
                }
            }
        }

        Ok(())
    }
}

impl<'a> Display for FIGure<'a> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for line in self.lines.iter() {
            write!(fmt, "{}\n", line)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::FIGure;
    use figfont::FIGfont;

    #[test]
    fn test_figure() {
        let font = FIGfont::standard().unwrap();
        let mut figure = FIGure::new(&font, 80);

        figure
            .add("Ciao ciao ciao ciao ciao ciao ciao ciao ciao ciao")
            .unwrap();

        println!("{}", figure);
    }
}
