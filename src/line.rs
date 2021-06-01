use figfont::{character::FIGcharacter, FIGfont};

pub struct FIGline<'a> {
    font: &'a FIGfont,
    chars: Vec<&'a FIGcharacter>,
}
