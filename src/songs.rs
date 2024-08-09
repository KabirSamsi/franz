/*  For now, just AST representations of what `franz-programs/<song>.fz`
should look like after parsing and full AST simplification. */

use crate::{
    ast,
    ast::{AExp::*, Accidental::*, BaseNoteLen::*, BasePitch::*}
};

/// Generates a [`crate::ast::NoteComp`] through a comma-separated list of
/// `[crate::ast::Note]`s.
macro_rules! notes {
    ($(($p1:expr, $p2:expr)),* $(,)?) => {
        $crate::ast::NoteComp::Phrase(vec![
            $(
                ($p1, $p2)
            ),*
        ])
    };
}

pub fn innocent() -> ast::NoteComp {
    notes![
        ((C, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(4)), (Eighth, 0)),
        ((E, Flat, Int(4)), (Qtr, 1)),
        ((D, Natural, Int(4)), (Eighth, 0)),
        ((E, Flat, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Half, 1)),
        ((G, Natural, Int(3)), (Qtr, 0)),
        ((C, Natural, Int(4)), (Qtr, 1)),
        ((B, Flat, Int(3)), (Eighth, 0)),
        ((C, Natural, Int(4)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Qtr, 0)),
        ((B, Flat, Int(3)), (Half, 1)),
        ((G, Natural, Int(3)), (Qtr, 0)),
        ((A, Flat, Int(3)), (Qtr, 1)),
        ((G, Natural, Int(3)), (Eighth, 0)),
        ((A, Flat, Int(3)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(3)), (Half, 1)),
        ((E, Flat, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Qtr, 1)),
        ((A, Natural, Int(3)), (Eighth, 0)),
        ((A, Natural, Int(3)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Half, 1)),
        ((C, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(4)), (Eighth, 0)),
        ((E, Flat, Int(4)), (Qtr, 1)),
        ((D, Natural, Int(4)), (Eighth, 0)),
        ((E, Flat, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Half, 1)),
        ((G, Natural, Int(3)), (Qtr, 0)),
        ((C, Natural, Int(4)), (Qtr, 1)),
        ((B, Flat, Int(3)), (Eighth, 0)),
        ((C, Natural, Int(4)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Qtr, 0)),
        ((B, Flat, Int(3)), (Half, 1)),
        ((G, Natural, Int(3)), (Qtr, 0)),
        ((A, Flat, Int(3)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(4)), (Qtr, 1)),
        ((E, Flat, Int(4)), (Qtr, 0)),
        ((F, Natural, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Eighth, 0)),
        ((E, Flat, Int(4)), (Eighth, 0)),
        ((E, Flat, Int(4)), (Half, 0)),
        ((E, Flat, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(4)), (Eighth, 0)),
        ((C, Natural, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Qtr, 0)),
        ((B, Natural, Int(3)), (Qtr, 0)),
        ((C, Natural, Int(4)), (Half, 1)),
        ((E, Flat, Int(4)), (Eighth, 0)),
        ((F, Natural, Int(4)), (Eighth, 0)),
        ((G, Natural, Int(4)), (Qtr, 1)),
        ((F, Natural, Int(4)), (Eighth, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((B, Flat, Int(4)), (Qtr, 0)),
        ((F, Natural, Int(4)), (Half, 1)),
        ((B, Flat, Int(3)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Qtr, 1)),
        ((D, Natural, Int(4)), (Eighth, 0)),
        ((E, Flat, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Half, 0)),
        ((C, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(4)), (Eighth, 0)),
        ((E, Flat, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Eighth, 0)),
        ((E, Flat, Int(4)), (Eighth, 0)),
        ((F, Natural, Int(4)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Qtr, 1)),
        ((B, Flat, Int(3)), (Eighth, 0)),
        ((B, Flat, Int(3)), (Half, 0)),
        ((A, Flat, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((F, Natural, Int(4)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Whole, 0)),
        ((C, Natural, Int(5)), (Half, 0)),
        ((B, Flat, Int(4)), (Half, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((F, Natural, Int(4)), (Eighth, 0)),
        ((E, Flat, Int(4)), (Eighth, 0)),
        ((E, Flat, Int(4)), (Half, 0)),
        ((F, Natural, Int(4)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Eighth, 0)),
        ((F, Natural, Int(4)), (Qtr, 1)),
        ((B, Flat, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Whole, 0)),
        ((C, Natural, Int(5)), (Half, 0)),
        ((B, Flat, Int(4)), (Half, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((F, Natural, Int(4)), (Eighth, 0)),
        ((E, Flat, Int(4)), (Eighth, 0)),
        ((E, Flat, Int(4)), (Half, 0)),
        ((F, Natural, Int(4)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Eighth, 0)),
        ((F, Natural, Int(4)), (Qtr, 1)),
        ((D, Natural, Int(4)), (Qtr, 0)),
        ((C, Natural, Int(4)), (Whole, 0)),
        ((C, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(4)), (Eighth, 0)),
        ((E, Flat, Int(4)), (Qtr, 1)),
        ((D, Natural, Int(4)), (Eighth, 0)),
        ((E, Flat, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Half, 1)),
        ((G, Natural, Int(3)), (Qtr, 0)),
        ((C, Natural, Int(4)), (Qtr, 1)),
        ((B, Flat, Int(3)), (Eighth, 0)),
        ((C, Natural, Int(4)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Qtr, 0)),
        ((B, Flat, Int(3)), (Half, 1)),
        ((G, Natural, Int(3)), (Qtr, 0)),
        ((A, Flat, Int(3)), (Qtr, 1)),
        ((G, Natural, Int(3)), (Eighth, 0)),
        ((A, Flat, Int(3)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(3)), (Half, 1)),
        ((E, Flat, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Qtr, 1)),
        ((A, Natural, Int(3)), (Eighth, 0)),
        ((A, Natural, Int(3)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Half, 1)),
        ((C, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(4)), (Eighth, 0)),
        ((E, Flat, Int(4)), (Qtr, 1)),
        ((D, Natural, Int(4)), (Eighth, 0)),
        ((E, Flat, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Half, 1)),
        ((G, Natural, Int(3)), (Qtr, 0)),
        ((C, Natural, Int(4)), (Qtr, 1)),
        ((B, Flat, Int(3)), (Eighth, 0)),
        ((C, Natural, Int(4)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Qtr, 0)),
        ((B, Flat, Int(3)), (Half, 1)),
        ((G, Natural, Int(3)), (Qtr, 0)),
        ((A, Flat, Int(3)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(4)), (Qtr, 1)),
        ((E, Flat, Int(4)), (Qtr, 0)),
        ((F, Natural, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Eighth, 0)),
        ((E, Flat, Int(4)), (Eighth, 0)),
        ((E, Flat, Int(4)), (Half, 0)),
        ((E, Flat, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(4)), (Eighth, 0)),
        ((C, Natural, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Qtr, 0)),
        ((B, Natural, Int(3)), (Qtr, 0)),
        ((C, Natural, Int(4)), (Half, 1))
    ]
}

pub fn anthem() -> ast::NoteComp {
    notes![
        ((F, Natural, Int(4)), (Eighth, 1)),
        ((D, Natural, Int(4)), (Sixteenth, 0)),
        ((B, Flat, Int(3)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Qtr, 0)),
        ((F, Natural, Int(4)), (Qtr, 0)),
        ((B, Flat, Int(4)), (Half, 0)),
        ((D, Natural, Int(5)), (Eighth, 1)),
        ((C, Natural, Int(5)), (Sixteenth, 0)),
        ((B, Flat, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Qtr, 0)),
        ((E, Natural, Int(4)), (Qtr, 0)),
        ((F, Natural, Int(4)), (Half, 0)),
        ((F, Natural, Int(4)), (Eighth, 0)),
        ((F, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Qtr, 1)),
        ((C, Natural, Int(5)), (Eighth, 0)),
        ((B, Flat, Int(4)), (Qtr, 0)),
        ((A, Natural, Int(4)), (Half, 0)),
        ((G, Natural, Int(4)), (Eighth, 1)),
        ((A, Natural, Int(4)), (Sixteenth, 0)),
        ((B, Flat, Int(4)), (Qtr, 0)),
        ((B, Flat, Int(4)), (Qtr, 0)),
        ((F, Natural, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Qtr, 0)),
        ((B, Flat, Int(3)), (Qtr, 0)),
        ((F, Natural, Int(4)), (Eighth, 1)),
        ((D, Natural, Int(4)), (Sixteenth, 0)),
        ((B, Flat, Int(3)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Qtr, 0)),
        ((F, Natural, Int(4)), (Qtr, 0)),
        ((B, Flat, Int(4)), (Half, 0)),
        ((D, Natural, Int(5)), (Eighth, 1)),
        ((C, Natural, Int(5)), (Sixteenth, 0)),
        ((B, Flat, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Qtr, 0)),
        ((E, Natural, Int(4)), (Qtr, 0)),
        ((F, Natural, Int(4)), (Half, 0)),
        ((F, Natural, Int(4)), (Eighth, 0)),
        ((F, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Qtr, 1)),
        ((C, Natural, Int(5)), (Eighth, 0)),
        ((B, Flat, Int(4)), (Qtr, 0)),
        ((A, Natural, Int(4)), (Half, 0)),
        ((G, Natural, Int(4)), (Eighth, 1)),
        ((A, Natural, Int(4)), (Sixteenth, 0)),
        ((B, Flat, Int(4)), (Qtr, 0)),
        ((B, Flat, Int(4)), (Qtr, 0)),
        ((F, Natural, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Qtr, 0)),
        ((B, Flat, Int(3)), (Qtr, 0)),
        ((D, Natural, Int(5)), (Eighth, 1)),
        ((D, Natural, Int(5)), (Sixteenth, 0)),
        ((D, Natural, Int(5)), (Qtr, 0)),
        ((E, Flat, Int(5)), (Qtr, 0)),
        ((F, Natural, Int(5)), (Qtr, 0)),
        ((F, Natural, Int(5)), (Half, 0)),
        ((E, Flat, Int(5)), (Eighth, 1)),
        ((D, Natural, Int(5)), (Sixteenth, 0)),
        ((C, Natural, Int(5)), (Qtr, 0)),
        ((D, Natural, Int(5)), (Qtr, 0)),
        ((E, Flat, Int(5)), (Qtr, 0)),
        ((E, Flat, Int(5)), (Half, 0)),
        ((E, Flat, Int(5)), (Qtr, 0)),
        ((D, Natural, Int(5)), (Qtr, 1)),
        ((C, Natural, Int(5)), (Eighth, 0)),
        ((B, Flat, Int(4)), (Qtr, 0)),
        ((A, Natural, Int(4)), (Half, 0)),
        ((G, Natural, Int(4)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Eighth, 1)),
        ((B, Flat, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Qtr, 0)),
        ((E, Natural, Int(4)), (Qtr, 0)),
        ((F, Natural, Int(4)), (Half, 0)),
        ((F, Natural, Int(4)), (Qtr, 0)),
        ((B, Flat, Int(4)), (Qtr, 0)),
        ((B, Flat, Int(4)), (Qtr, 0)),
        ((B, Flat, Int(4)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((C, Natural, Int(5)), (Qtr, 0)),
        ((E, Flat, Int(5)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Eighth, 0)),
        ((C, Natural, Int(5)), (Eighth, 0)),
        ((B, Flat, Int(4)), (Eighth, 0)),
        ((B, Flat, Int(4)), (Qtr, 0)),
        ((A, Natural, Int(4)), (Qtr, 0)),
        ((F, Natural, Int(4)), (Eighth, 0)),
        ((F, Natural, Int(4)), (Eighth, 0)),
        ((B, Flat, Int(4)), (Qtr, 1)),
        ((C, Natural, Int(5)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Eighth, 0)),
        ((E, Flat, Int(5)), (Eighth, 0)),
        ((F, Natural, Int(5)), (Half, 0)),
        ((B, Flat, Int(4)), (Eighth, 0)),
        ((C, Natural, Int(5)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Qtr, 1)),
        ((E, Flat, Int(5)), (Eighth, 0)),
        ((C, Natural, Int(5)), (Qtr, 0)),
        ((B, Flat, Int(4)), (Whole, 0)),
    ]
}

pub fn apprasionata() -> ast::NoteComp {
    notes![
        ((C, Natural, Int(3)), (Qtr, 0)),
        ((F, Natural, Int(3)), (Qtr, 0)),
        ((A, Flat, Int(3)), (Qtr, 0)),
        ((C, Natural, Int(4)), (Qtr, 0)),
        ((F, Natural, Int(4)), (Qtr, 0)),
        ((A, Flat, Int(4)), (Qtr, 1)),
        ((G, Natural, Int(4)), (Sixteenth, 0)),
        ((F, Natural, Int(4)), (Sixteenth, 0)),
        ((E, Natural, Int(4)), (Sixteenth, 0)),
        ((F, Natural, Int(4)), (Half, 0)),
        ((G, Natural, Int(3)), (Qtr, 0)),
        ((C, Natural, Int(4)), (Qtr, 0)),
        ((E, Natural, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((B, Flat, Int(4)), (Qtr, 1)),
        ((A, Flat, Int(4)), (Sixteenth, 0)),
        ((G, Natural, Int(4)), (Sixteenth, 0)),
        ((F, Natural, Int(4)), (Sixteenth, 0)),
        ((G, Natural, Int(4)), (Half, 0)),
        ((A, Flat, Int(4)), (Qtr, 1)),
        ((G, Natural, Int(4)), (Sixteenth, 0)),
        ((F, Natural, Int(4)), (Sixteenth, 0)),
        ((E, Natural, Int(4)), (Sixteenth, 0)),
        ((F, Natural, Int(4)), (Half, 0)),
        ((B, Flat, Int(4)), (Qtr, 1)),
        ((A, Flat, Int(4)), (Sixteenth, 0)),
        ((G, Natural, Int(4)), (Sixteenth, 0)),
        ((F, Natural, Int(4)), (Sixteenth, 0)),
        ((G, Natural, Int(4)), (Half, 0)),
        ((C, Natural, Int(5)), (Half, 0)),
        ((B, Flat, Int(4)), (Eighth, 0)),
        ((A, Flat, Int(4)), (Eighth, 0)),
        ((G, Natural, Int(4)), (Eighth, 0)),
        ((F, Natural, Int(4)), (Eighth, 1)),
        ((E, Natural, Int(4)), (Sixteenth, 0)),
        ((F, Natural, Int(4)), (Sixteenth, 0)),
        ((G, Natural, Int(4)), (Sixteenth, 0)),
        ((F, Natural, Int(4)), (Qtr, 1)),
        ((E, Natural, Int(4)), (Half, 0)),
    ]
}

pub fn anthem2() -> ast::NoteComp {
    notes![
        ((G, Natural, Int(4)), (Eighth, 0)),
        ((C, Natural, Int(5)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Eighth, 1)),
        ((A, Natural, Int(4)), (Sixteenth, 0)),
        ((B, Natural, Int(4)), (Qtr, 0)),
        ((E, Natural, Int(4)), (Eighth, 0)),
        ((E, Natural, Int(4)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Eighth, 1)),
        ((F, Natural, Int(4)), (Sixteenth, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((C, Natural, Int(4)), (Eighth, 0)),
        ((C, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Eighth, 1)),
        ((E, Natural, Int(4)), (Sixteenth, 0)),
        ((F, Natural, Int(4)), (Qtr, 0)),
        ((F, Natural, Int(4)), (Eighth, 0)),
        ((G, Natural, Int(4)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Qtr, 0)),
        ((B, Natural, Int(4)), (Eighth, 1)),
        ((C, Natural, Int(5)), (Sixteenth, 0)),
        ((D, Natural, Int(5)), (Qtr, 1)),
        ((G, Natural, Int(4)), (Eighth, 0)),
        ((E, Natural, Int(5)), (Qtr, 0)),
        ((D, Natural, Int(5)), (Eighth, 1)),
        ((C, Natural, Int(5)), (Sixteenth, 0)),
        ((D, Natural, Int(5)), (Qtr, 0)),
        ((B, Natural, Int(4)), (Eighth, 0)),
        ((G, Natural, Int(4)), (Eighth, 0)),
        ((C, Natural, Int(5)), (Qtr, 0)),
        ((B, Natural, Int(4)), (Eighth, 1)),
        ((A, Natural, Int(4)), (Sixteenth, 0)),
        ((B, Natural, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Eighth, 0)),
        ((E, Natural, Int(4)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Eighth, 1)),
        ((F, Natural, Int(4)), (Sixteenth, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((E, Natural, Int(4)), (Eighth, 0)),
        ((C, Natural, Int(4)), (Eighth, 0)),
        ((C, Natural, Int(5)), (Qtr, 0)),
        ((B, Natural, Int(4)), (Eighth, 1)),
        ((A, Natural, Int(4)), (Sixteenth, 0)),
        ((G, Natural, Int(4)), (Half, 0)),
        ((E, Natural, Int(5)), (Half, 0)),
        ((D, Natural, Int(5)), (Eighth, 0)),
        ((C, Natural, Int(5)), (Eighth, 0)),
        ((B, Natural, Int(4)), (Eighth, 0)),
        ((C, Natural, Int(5)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Qtr, 1)),
        ((B, Natural, Int(4)), (Eighth, 0)),
        ((G, Natural, Int(4)), (Half, 0)),
        ((C, Natural, Int(5)), (Half, 0)),
        ((B, Natural, Int(4)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((G, Natural, Int(4)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((B, Natural, Int(4)), (Qtr, 1)),
        ((G, Natural, Int(4)), (Eighth, 0)),
        ((E, Natural, Int(4)), (Half, 0)),
        ((C, Natural, Int(5)), (Qtr, 0)),
        ((A, Natural, Int(4)), (Eighth, 1)),
        ((B, Natural, Int(4)), (Sixteenth, 0)),
        ((C, Natural, Int(5)), (Qtr, 0)),
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((B, Natural, Int(4)), (Eighth, 0)),
        ((C, Natural, Int(5)), (Qtr, 0)),
        ((A, Natural, Int(4)), (Eighth, 1)),
        ((C, Natural, Int(5)), (Sixteenth, 0)),
        ((F, Natural, Int(5)), (Half, 0)),
        ((F, Natural, Int(5)), (Half, 0)),
        ((E, Natural, Int(5)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Eighth, 0)),
        ((C, Natural, Int(5)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Eighth, 0)),
        ((E, Natural, Int(5)), (Qtr, 1)),
        ((C, Natural, Int(5)), (Eighth, 0)),
        ((G, Natural, Int(4)), (Half, 0)),
        ((D, Natural, Int(5)), (Half, 0)),
        ((C, Natural, Int(5)), (Eighth, 0)),
        ((B, Natural, Int(4)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((B, Natural, Int(4)), (Eighth, 0)),
        ((C, Natural, Int(5)), (Qtr, 1)),
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Half, 0)),
        ((C, Natural, Int(5)), (Qtr, 0)),
        ((B, Natural, Int(4)), (Eighth, 1)),
        ((A, Natural, Int(4)), (Sixteenth, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((E, Natural, Int(4)), (Eighth, 0)),
        ((C, Natural, Int(4)), (Eighth, 0)),
        ((G, Natural, Int(4)), (Half, 0)),
        ((A, Natural, Int(4)), (Qtr, 0)),
        ((B, Natural, Int(4)), (Qtr, 0)),
        ((C, Natural, Int(5)), (Whole, 0)),
    ]
}

pub fn cantina() -> ast::NoteComp {
    notes![
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Qtr, 0)),
        ((A, Natural, Int(4)), (Sixteenth, 0)),
        ((G, Sharp, Int(4)), (Sixteenth, 0)),
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((G, Natural, Int(4)), (Sixteenth, 0)),
        ((F, Sharp, Int(4)), (Sixteenth, 0)),
        ((G, Natural, Int(4)), (Eighth, 0)),
        ((F, Natural, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Qtr, 0)),
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Qtr, 0)),
        ((C, Natural, Int(5)), (Sixteenth, 0)),
        ((B, Natural, Int(4)), (Sixteenth, 0)),
        ((C, Natural, Int(5)), (Eighth, 0)),
        ((B, Flat, Int(4)), (Sixteenth, 0)),
        ((A, Natural, Int(4)), (Sixteenth, 0)),
        ((B, Flat, Int(4)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Qtr, 0)),
        ((E, Natural, Int(4)), (Qtr, 0)),
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Qtr, 0)),
        ((A, Natural, Int(4)), (Sixteenth, 0)),
        ((G, Sharp, Int(4)), (Sixteenth, 0)),
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((G, Natural, Int(4)), (Sixteenth, 0)),
        ((F, Sharp, Int(4)), (Sixteenth, 0)),
        ((G, Natural, Int(4)), (Eighth, 0)),
        ((F, Natural, Int(4)), (Qtr, 0)),
        ((D, Natural, Int(4)), (Qtr, 0)),
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Eighth, 0)),
        ((D, Natural, Int(5)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Qtr, 0)),
        ((C, Natural, Int(5)), (Sixteenth, 0)),
        ((B, Natural, Int(4)), (Sixteenth, 0)),
        ((C, Natural, Int(5)), (Eighth, 0)),
        ((B, Flat, Int(4)), (Sixteenth, 0)),
        ((A, Natural, Int(4)), (Sixteenth, 0)),
        ((B, Flat, Int(4)), (Eighth, 0)),
        ((A, Natural, Int(4)), (Qtr, 0)),
        ((E, Natural, Int(4)), (Qtr, 0))
    ]
}

pub fn imperial_march() -> ast::NoteComp {
    notes![
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Eighth, 1)),
        ((B, Flat, Int(4)), (Sixteenth, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Eighth, 1)),
        ((B, Flat, Int(4)), (Sixteenth, 0)),
        ((G, Natural, Int(4)), (Half, 0)),
        ((D, Natural, Int(5)), (Qtr, 0)),
        ((D, Natural, Int(5)), (Qtr, 0)),
        ((D, Natural, Int(5)), (Qtr, 0)),
        ((E, Flat, Int(5)), (Eighth, 1)),
        ((B, Flat, Int(4)), (Sixteenth, 0)),
        ((F, Sharp, Int(4)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Eighth, 1)),
        ((B, Flat, Int(4)), (Sixteenth, 0)),
        ((G, Natural, Int(4)), (Half, 0)),
        ((G, Natural, Int(5)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Eighth, 1)),
        ((G, Natural, Int(4)), (Sixteenth, 0)),
        ((G, Natural, Int(5)), (Qtr, 0)),
        ((F, Sharp, Int(5)), (Eighth, 1)),
        ((F, Natural, Int(5)), (Sixteenth, 0)),
        ((E, Natural, Int(5)), (Sixteenth, 0)),
        ((D, Sharp, Int(5)), (Sixteenth, 0)),
        ((E, Natural, Int(5)), (Qtr, 0)),
        ((G, Sharp, Int(4)), (Eighth, 0)),
        ((C, Sharp, Int(5)), (Qtr, 0)),
        ((C, Natural, Int(5)), (Eighth, 1)),
        ((B, Natural, Int(4)), (Sixteenth, 0)),
        ((B, Flat, Int(4)), (Sixteenth, 0)),
        ((A, Natural, Int(4)), (Sixteenth, 0)),
        ((B, Flat, Int(4)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Eighth, 0)),
        ((G, Flat, Int(4)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Eighth, 1)),
        ((G, Flat, Int(4)), (Sixteenth, 0)),
        ((B, Flat, Int(4)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Eighth, 1)),
        ((B, Flat, Int(4)), (Sixteenth, 0)),
        ((D, Natural, Int(5)), (Half, 0)),
        ((G, Natural, Int(5)), (Qtr, 0)),
        ((G, Natural, Int(4)), (Eighth, 1)),
        ((G, Natural, Int(4)), (Sixteenth, 0)),
        ((G, Natural, Int(5)), (Qtr, 0)),
        ((F, Sharp, Int(5)), (Eighth, 1)),
        ((F, Natural, Int(5)), (Sixteenth, 0)),
        ((E, Natural, Int(5)), (Sixteenth, 0)),
        ((D, Sharp, Int(5)), (Sixteenth, 0)),
        ((E, Natural, Int(5)), (Qtr, 0)),
        ((G, Sharp, Int(4)), (Eighth, 0)),
        ((C, Sharp, Int(5)), (Qtr, 0)),
        ((C, Natural, Int(5)), (Eighth, 1)),
        ((B, Natural, Int(4)), (Sixteenth, 0)),
        ((B, Flat, Int(4)), (Sixteenth, 0)),
        ((A, Natural, Int(4)), (Sixteenth, 0)),
        ((B, Flat, Int(4)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Eighth, 0)),
        ((G, Flat, Int(4)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Eighth, 1)),
        ((B, Flat, Int(4)), (Sixteenth, 0)),
        ((G, Natural, Int(4)), (Qtr, 0)),
        ((E, Flat, Int(4)), (Eighth, 1)),
        ((B, Flat, Int(4)), (Sixteenth, 0)),
        ((G, Natural, Int(4)), (Half, 0)),
    ]
}
