use std::{
    borrow::Borrow,
    fmt::{Display, Formatter},
};

use figfont::{header::Layout, subcharacter::SubCharacter, FIGfont};

#[derive(Clone)]
pub struct FIGline<'a> {
    font: &'a FIGfont,
    chars: Vec<i32>,
    lines: Vec<Vec<SubCharacter>>,
}

#[inline]
fn is_space(c: &SubCharacter) -> bool {
    match c {
        SubCharacter::Symbol(r) => r == " ",
        _ => false,
    }
}

#[inline]
fn _max_kerning(c1: &Vec<SubCharacter>, c2: &Vec<SubCharacter>) -> usize {
    let mut k1 = 0;
    for sch in c1.iter().rev() {
        if is_space(sch) {
            k1 += 1
        } else {
            break;
        }
    }

    let mut k2 = 0;
    for sch in c2.iter() {
        if is_space(sch) {
            k2 += 1;
        } else {
            break;
        }
    }

    k1 + k2
}

#[inline]
fn max_kerning(c1: &Vec<Vec<SubCharacter>>, c2: &Vec<Vec<SubCharacter>>) -> usize {
    let mut kern = c1[0].len();

    for i in 0..c1.len() {
        kern = std::cmp::min(kern, _max_kerning(&c1[i], &c2[i]));
    }

    kern
}

#[inline]
fn __apply_kerning(c1: &mut Vec<SubCharacter>, c2: &mut Vec<SubCharacter>, mut n: usize) {
    while !c1.is_empty() && is_space(c1.last().unwrap()) && n > 0 {
        c1.truncate(c1.len() - 1);
        n -= 1;
    }

    while !c2.is_empty() && is_space(c2.first().unwrap()) && n > 0 {
        c2.remove(0);
        n -= 1;
    }
}

#[inline]
fn _apply_kerning(c1: &mut Vec<Vec<SubCharacter>>, c2: &mut Vec<Vec<SubCharacter>>, n: usize) {
    for i in 0..c1.len() {
        __apply_kerning(&mut c1[i], &mut c2[i], n);
    }
}

#[inline]
fn apply_kerning(c1: &mut Vec<Vec<SubCharacter>>, c2: &mut Vec<Vec<SubCharacter>>) {
    let kern = max_kerning(c1, c2);

    if kern > 0 {
        _apply_kerning(c1, c2, kern);
    }
}

#[inline]
fn needs_smushing(layout: Layout) -> bool {
    layout.contains(Layout::HORIZONTAL_SMUSH)
}

#[inline]
fn needs_kerning(layout: Layout) -> bool {
    layout.contains(Layout::HORIZONTAL_KERNING) || needs_smushing(layout)
}

fn equal_smush(c1: &SubCharacter, c2: &SubCharacter) -> Option<SubCharacter> {
    if c1.is_blank() || c2.is_blank() {
        None
    } else if c1 == c2 {
        Some(c1.clone())
    } else {
        None
    }
}

fn underscore_smush(c1: &SubCharacter, c2: &SubCharacter) -> Option<SubCharacter> {
    let underscore = SubCharacter::Symbol("_".to_string());
    let replacing: Vec<SubCharacter> = "|/\\[]{}()<>"
        .chars()
        .map(|c| SubCharacter::Symbol(c.to_string()))
        .collect();

    if c1 == &underscore && replacing.contains(c2) {
        Some(c2.clone())
    } else if c2 == &underscore && replacing.contains(c1) {
        Some(c1.clone())
    } else {
        None
    }
}

fn hierarchy_class(c: &str) -> Option<usize> {
    match c {
        "|" => Some(1),
        "/" | "\\" => Some(2),
        "[" | "]" => Some(3),
        "{" | "}" => Some(4),
        "(" | ")" => Some(5),
        "<" | ">" => Some(6),
        _ => None,
    }
}

fn hierarchy_smush(c1: &SubCharacter, c2: &SubCharacter) -> Option<SubCharacter> {
    if c1.is_blank() || c2.is_blank() {
        return None;
    }

    let k1 = hierarchy_class(c1.borrow())?;
    let k2 = hierarchy_class(c2.borrow())?;

    if k1 > k2 {
        Some(c1.clone())
    } else if k2 > k1 {
        Some(c2.clone())
    } else {
        None
    }
}

fn opposite_smush(c1: &SubCharacter, c2: &SubCharacter) -> Option<SubCharacter> {
    let pairs = &[("[", "]"), ("{", "}"), ("(", ")")];

    if c1.is_blank() || c2.is_blank() {
        return None;
    }

    let c1: &str = c1.borrow();
    let c2: &str = c2.borrow();

    for (p1, p2) in pairs {
        if (c1 == *p1 && c2 == *p2) || (c1 == *p2 && c2 == *p1) {
            return Some(SubCharacter::Symbol("|".to_string()));
        }
    }

    None
}

fn bigx_smush(c1: &SubCharacter, c2: &SubCharacter) -> Option<SubCharacter> {
    if c1.is_blank() || c2.is_blank() {
        return None;
    }

    let c1: &str = c1.borrow();
    let c2: &str = c2.borrow();

    match (c1, c2) {
        ("/", "\\") => Some(SubCharacter::Symbol("|".to_string())),
        ("\\", "/") => Some(SubCharacter::Symbol("Y".to_string())),
        (">", "<") => Some(SubCharacter::Symbol("X".to_string())),
        _ => None,
    }
}

fn hardblank_smush(c1: &SubCharacter, c2: &SubCharacter) -> Option<SubCharacter> {
    if c1.is_blank() && c2.is_blank() {
        Some(SubCharacter::Blank)
    } else {
        None
    }
}

fn space_smush(c1: &SubCharacter, c2: &SubCharacter) -> Option<SubCharacter> {
    if c1.is_blank() || c2.is_blank() {
        return None;
    }

    if is_space(c1) {
        Some(c2.clone())
    } else if is_space(c2) {
        Some(c1.clone())
    } else {
        None
    }
}

fn get_smush_char(
    line1: &mut Vec<SubCharacter>,
    line2: &Vec<SubCharacter>,
    layout: Layout,
) -> Option<SubCharacter> {
    macro_rules! ret_some {
        ($e:expr) => {
            match $e {
                Some(x) => return Some(x),
                None => (),
            }
        };
    }

    macro_rules! apply {
        ($fn:ident) => {
            ret_some!($fn(line1.last().unwrap(), line2.first().unwrap()))
        };
        ($fn:ident, $cond:ident) => {
            if layout.contains(Layout::$cond) {
                apply!($fn);
            }
        };
    }

    apply!(space_smush);
    apply!(equal_smush, HORIZONTAL_EQUAL);
    apply!(underscore_smush, HORIZONTAL_LOWLINE);
    apply!(hierarchy_smush, HORIZONTAL_HIERARCHY);
    apply!(opposite_smush, HORIZONTAL_PAIR);
    apply!(bigx_smush, HORIZONTAL_BIGX);
    apply!(hardblank_smush, HORIZONTAL_HARDBLANK);

    None
}

fn apply_smushing(
    ch1: &mut Vec<Vec<SubCharacter>>,
    mut ch2: Vec<Vec<SubCharacter>>,
    layout: Layout,
) {
    let mut smush_chars: Vec<Option<SubCharacter>> = Vec::with_capacity(ch1.len());
    if needs_smushing(layout) {
        for i in 0..ch1.len() {
            smush_chars.push(get_smush_char(&mut ch1[i], &ch2[i], layout));
        }
    } else {
        for _ in 0..ch1.len() {
            smush_chars.push(None);
        }
    }

    if smush_chars.iter().all(Option::is_some) {
        let smush_chars: Vec<SubCharacter> = smush_chars.into_iter().map(Option::unwrap).collect();

        for i in 0..ch1.len() {
            ch2[i].remove(0);
            let l = ch1[i].len();
            ch1[i].truncate(l - 1);
            ch1[i].push(smush_chars[i].clone());

            for sch in ch2[i].iter() {
                ch1[i].push(sch.clone());
            }
        }
    } else {
        for i in 0..ch1.len() {
            for sch in ch2[i].iter() {
                ch1[i].push(sch.clone());
            }
        }
    }
}

impl<'a> FIGline<'a> {
    pub fn new<'b>(font: &'b FIGfont) -> FIGline<'b> {
        let mut lines: Vec<Vec<SubCharacter>> = Vec::with_capacity(font.header().height());
        for _ in 0..font.header().height() {
            lines.push(Vec::new());
        }

        FIGline {
            font,
            chars: Vec::new(),
            lines,
        }
    }

    pub fn add_char(&mut self, ch: i32) {
        let is_empty = self.chars.is_empty();
        self.chars.push(ch);
        if is_empty || !needs_kerning(self.font.header().layout()) {
            let ch = self.font.get(ch);
            for (i, line) in ch.lines().iter().enumerate() {
                for sch in line {
                    self.lines[i].push(sch.clone());
                }
            }
        } else {
            let mut ch = self.font.get(ch).lines().into_owned();
            apply_kerning(&mut self.lines, &mut ch);
            apply_smushing(&mut self.lines, ch, self.font.header().layout());
        }
    }

    pub fn add_line(&mut self, line: &FIGline) {
        if self.is_empty() || !needs_kerning(self.font.header().layout()) {
            for c in line.chars.iter() {
                self.chars.push(*c);
            }

            for (i, line) in line.lines.iter().enumerate() {
                for sch in line {
                    self.lines[i].push(sch.clone());
                }
            }
        } else {
            for c in line.chars.iter() {
                self.chars.push(*c);
            }
            let mut ch = line.lines.clone();
            apply_kerning(&mut self.lines, &mut ch);
            apply_smushing(&mut self.lines, ch, self.font.header().layout());
        }
    }

    pub fn width(&self) -> usize {
        self.lines
            .iter()
            .map(|line| line.iter().map(SubCharacter::width).sum::<usize>())
            .max()
            .unwrap_or(0)
    }

    pub fn height(&self) -> usize {
        self.font.header().height()
    }

    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }
}

impl<'a> Display for FIGline<'a> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let res: Vec<String> = self
            .lines
            .iter()
            .map(|line| line.iter().map(ToString::to_string).collect::<String>())
            .collect();

        write!(fmt, "{}", res.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::FIGline;
    use encoding::{all::ISO_8859_1, Encoding};
    use figfont::FIGfont;

    #[test]
    fn test() {
        let font = FIGfont::standard().unwrap();
        let mut line = FIGline::new(&font);
        let chars = ISO_8859_1
            .encode("CiT", encoding::EncoderTrap::Replace)
            .unwrap();
        for c in chars {
            line.add_char(c as i32);
        }

        println!("{}", line);
    }
}
