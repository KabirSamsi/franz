/*  For now, just AST representations of what `franz-programs/<song>.fz`
    should look like after parsing and full AST simplification. */

use crate::ast;
use crate::ast::BasePitch::*;
use crate::ast::Accidental::*;
use crate::ast::BaseNoteLen::*;

pub fn innocent() -> ast::Expr {
    let mut vec = Vec::new();

    vec.push(((C, Natural, 4), (Eighth, 0)));
    vec.push(((D, Natural, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Qtr, 1)));
    vec.push(((D, Natural, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((G, Natural, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Half, 1)));

    vec.push(((G, Natural, 3), (Qtr, 0)));
    vec.push(((C, Natural, 4), (Qtr, 1)));
    vec.push(((B, Flat, 3), (Eighth, 0)));
    vec.push(((C, Natural, 4), (Qtr, 0)));
    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((B, Flat, 3), (Half, 1)));

    vec.push(((G, Natural, 3), (Qtr, 0)));
    vec.push(((A, Flat, 3), (Qtr, 1)));
    vec.push(((G, Natural, 3), (Eighth, 0)));
    vec.push(((A, Flat, 3), (Qtr, 0)));
    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((G, Natural, 3), (Half, 1)));

    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Qtr, 1)));
    vec.push(((A, Natural, 3), (Eighth, 0)));
    vec.push(((A, Natural, 3), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Half, 1)));

    vec.push(((C, Natural, 4), (Eighth, 0)));
    vec.push(((D, Natural, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Qtr, 1)));
    vec.push(((D, Natural, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((G, Natural, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Half, 1)));

    vec.push(((G, Natural, 3), (Qtr, 0)));
    vec.push(((C, Natural, 4), (Qtr, 1)));
    vec.push(((B, Flat, 3), (Eighth, 0)));
    vec.push(((C, Natural, 4), (Qtr, 0)));
    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((B, Flat, 3), (Half, 1)));

    vec.push(((G, Natural, 3), (Qtr, 0)));
    vec.push(((A, Flat, 3), (Qtr, 0)));
    vec.push(((E, Flat, 4), (Eighth, 0)));
    vec.push(((D, Natural, 4), (Qtr, 1)));
    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((F, Natural, 4), (Qtr, 0)));
    vec.push(((G, Natural, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Half, 0)));

    vec.push(((E, Flat, 4), (Eighth, 0)));
    vec.push(((D, Natural, 4), (Eighth, 0)));
    vec.push(((C, Natural, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Qtr, 0)));
    vec.push(((B, Natural, 3), (Qtr, 0)));
    vec.push(((C, Natural, 4), (Half, 1)));

    vec.push(((E, Flat, 4), (Eighth, 0)));
    vec.push(((F, Natural, 4), (Eighth, 0)));
    vec.push(((G, Natural, 4), (Qtr, 1)));
    vec.push(((F, Natural, 4), (Eighth, 0)));
    vec.push(((G, Natural, 4), (Qtr, 0)));
    vec.push(((B, Flat, 4), (Qtr, 0)));
    vec.push(((F, Natural, 4), (Half, 1)));

    vec.push(((B, Flat, 3), (Qtr, 0)));
    vec.push(((E, Flat, 4), (Qtr, 1)));
    vec.push(((D, Natural, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((G, Natural, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Half, 0)));

    vec.push(((C, Natural, 4), (Eighth, 0)));
    vec.push(((D, Natural, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Eighth, 0)));
    vec.push(((F, Natural, 4), (Qtr, 0)));
    vec.push(((E, Flat, 4), (Qtr, 1)));
    vec.push(((B, Flat, 3), (Eighth, 0)));
    vec.push(((B, Flat, 3), (Half, 0)));

    vec.push(((A, Flat, 4), (Qtr, 0)));
    vec.push(((G, Natural, 4), (Qtr, 0)));
    vec.push(((F, Natural, 4), (Qtr, 0)));
    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((G, Natural, 4), (Whole, 0)));

    vec.push(((C, Natural, 5), (Half, 0)));
    vec.push(((B, Flat, 4), (Half, 0)));
    vec.push(((G, Natural, 4), (Qtr, 0)));
    vec.push(((F, Natural, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Half, 0)));

    vec.push(((F, Natural, 4), (Qtr, 0)));
    vec.push(((E, Flat, 4), (Eighth, 0)));
    vec.push(((F, Natural, 4), (Qtr, 1)));
    vec.push(((B, Flat, 4), (Qtr, 0)));
    vec.push(((G, Natural, 4), (Whole, 0)));

    vec.push(((C, Natural, 5), (Half, 0)));
    vec.push(((B, Flat, 4), (Half, 0)));
    vec.push(((G, Natural, 4), (Qtr, 0)));
    vec.push(((F, Natural, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Half, 0)));

    vec.push(((F, Natural, 4), (Qtr, 0)));
    vec.push(((E, Flat, 4), (Eighth, 0)));
    vec.push(((F, Natural, 4), (Qtr, 1)));
    vec.push(((D, Natural, 4), (Qtr, 0)));
    vec.push(((C, Natural, 4), (Whole, 0)));
    
    vec.push(((C, Natural, 4), (Eighth, 0)));
    vec.push(((D, Natural, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Qtr, 1)));
    vec.push(((D, Natural, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((G, Natural, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Half, 1)));

    vec.push(((G, Natural, 3), (Qtr, 0)));
    vec.push(((C, Natural, 4), (Qtr, 1)));
    vec.push(((B, Flat, 3), (Eighth, 0)));
    vec.push(((C, Natural, 4), (Qtr, 0)));
    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((B, Flat, 3), (Half, 1)));

    vec.push(((G, Natural, 3), (Qtr, 0)));
    vec.push(((A, Flat, 3), (Qtr, 1)));
    vec.push(((G, Natural, 3), (Eighth, 0)));
    vec.push(((A, Flat, 3), (Qtr, 0)));
    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((G, Natural, 3), (Half, 1)));

    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Qtr, 1)));
    vec.push(((A, Natural, 3), (Eighth, 0)));
    vec.push(((A, Natural, 3), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Half, 1)));

    vec.push(((C, Natural, 4), (Eighth, 0)));
    vec.push(((D, Natural, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Qtr, 1)));
    vec.push(((D, Natural, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((G, Natural, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Half, 1)));

    vec.push(((G, Natural, 3), (Qtr, 0)));
    vec.push(((C, Natural, 4), (Qtr, 1)));
    vec.push(((B, Flat, 3), (Eighth, 0)));
    vec.push(((C, Natural, 4), (Qtr, 0)));
    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((B, Flat, 3), (Half, 1)));

    vec.push(((G, Natural, 3), (Qtr, 0)));
    vec.push(((A, Flat, 3), (Qtr, 0)));
    vec.push(((E, Flat, 4), (Eighth, 0)));
    vec.push(((D, Natural, 4), (Qtr, 1)));
    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((F, Natural, 4), (Qtr, 0)));
    vec.push(((G, Natural, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Half, 0)));

    vec.push(((E, Flat, 4), (Eighth, 0)));
    vec.push(((D, Natural, 4), (Eighth, 0)));
    vec.push(((C, Natural, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Qtr, 0)));
    vec.push(((B, Natural, 3), (Qtr, 0)));
    vec.push(((C, Natural, 4), (Half, 1)));

    return ast::Expr::MusicSeq(vec);
}

pub fn anthem() -> ast::Expr {
    let mut vec = Vec::new();

    vec.push(((F, Natural, 4), (Eighth, 1)));
    vec.push(((D, Natural, 4), (Sixteenth, 0)));
    vec.push(((B, Flat, 3), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Qtr, 0)));
    vec.push(((F, Natural, 4), (Qtr, 0)));
    vec.push(((B, Flat, 4), (Half, 0)));

    vec.push(((D, Natural, 5), (Eighth, 1)));
    vec.push(((C, Natural, 5), (Sixteenth, 0)));
    vec.push(((B, Flat, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Qtr, 0)));
    vec.push(((E, Natural, 4), (Qtr, 0)));
    vec.push(((F, Natural, 4), (Half, 0)));

    vec.push(((F, Natural, 4), (Eighth, 0)));
    vec.push(((F, Natural, 4), (Eighth, 0)));
    vec.push(((D, Natural, 5), (Qtr, 1)));
    vec.push(((C, Natural, 5), (Eighth, 0)));
    vec.push(((B, Flat, 4), (Qtr, 0)));
    vec.push(((A, Natural, 4), (Half, 0)));

    vec.push(((G, Natural, 4), (Eighth, 1)));
    vec.push(((A, Natural, 4), (Sixteenth, 0)));
    vec.push(((B, Flat, 4), (Qtr, 0)));
    vec.push(((B, Flat, 4), (Qtr, 0)));
    vec.push(((F, Natural, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Qtr, 0)));
    vec.push(((B, Flat, 3), (Qtr, 0)));

    vec.push(((F, Natural, 4), (Eighth, 1)));
    vec.push(((D, Natural, 4), (Sixteenth, 0)));
    vec.push(((B, Flat, 3), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Qtr, 0)));
    vec.push(((F, Natural, 4), (Qtr, 0)));
    vec.push(((B, Flat, 4), (Half, 0)));

    vec.push(((D, Natural, 5), (Eighth, 1)));
    vec.push(((C, Natural, 5), (Sixteenth, 0)));
    vec.push(((B, Flat, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Qtr, 0)));
    vec.push(((E, Natural, 4), (Qtr, 0)));
    vec.push(((F, Natural, 4), (Half, 0)));

    vec.push(((F, Natural, 4), (Eighth, 0)));
    vec.push(((F, Natural, 4), (Eighth, 0)));
    vec.push(((D, Natural, 5), (Qtr, 1)));
    vec.push(((C, Natural, 5), (Eighth, 0)));
    vec.push(((B, Flat, 4), (Qtr, 0)));
    vec.push(((A, Natural, 4), (Half, 0)));

    vec.push(((G, Natural, 4), (Eighth, 1)));
    vec.push(((A, Natural, 4), (Sixteenth, 0)));
    vec.push(((B, Flat, 4), (Qtr, 0)));
    vec.push(((B, Flat, 4), (Qtr, 0)));
    vec.push(((F, Natural, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Qtr, 0)));
    vec.push(((B, Flat, 3), (Qtr, 0)));

    vec.push(((D, Natural, 5), (Eighth, 1)));
    vec.push(((D, Natural, 5), (Sixteenth, 0)));
    vec.push(((D, Natural, 5), (Qtr, 0)));
    vec.push(((E, Flat, 5), (Qtr, 0)));
    vec.push(((F, Natural, 5), (Qtr, 0)));
    vec.push(((F, Natural, 5), (Half, 0)));

    vec.push(((E, Flat, 5), (Eighth, 1)));
    vec.push(((D, Natural, 5), (Sixteenth, 0)));
    vec.push(((C, Natural, 5), (Qtr, 0)));
    vec.push(((D, Natural, 5), (Qtr, 0)));
    vec.push(((E, Flat, 5), (Qtr, 0)));
    vec.push(((E, Flat, 5), (Half, 0)));

    vec.push(((E, Flat, 5), (Qtr, 0)));
    vec.push(((D, Natural, 5), (Qtr, 1)));
    vec.push(((C, Natural, 5), (Eighth, 0)));
    vec.push(((B, Flat, 4), (Qtr, 0)));
    vec.push(((A, Natural, 4), (Half, 0)));

    vec.push(((G, Natural, 4), (Eighth, 0)));
    vec.push(((A, Natural, 4), (Eighth, 1)));
    vec.push(((B, Flat, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Qtr, 0)));
    vec.push(((E, Natural, 4), (Qtr, 0)));
    vec.push(((F, Natural, 4), (Half, 0)));

    vec.push(((F, Natural, 4), (Qtr, 0)));
    vec.push(((B, Flat, 4), (Qtr, 0)));
    vec.push(((B, Flat, 4), (Qtr, 0)));
    vec.push(((B, Flat, 4), (Eighth, 0)));
    vec.push(((A, Natural, 4), (Eighth, 0)));
    vec.push(((G, Natural, 4), (Qtr, 0)));
    vec.push(((G, Natural, 4), (Qtr, 0)));
    vec.push(((G, Natural, 4), (Qtr, 0)));
    vec.push(((C, Natural, 5), (Qtr, 0)));
    vec.push(((E, Flat, 5), (Eighth, 0)));
    vec.push(((D, Natural, 5), (Eighth, 0)));
    vec.push(((C, Natural, 5), (Eighth, 0)));
    vec.push(((B, Flat, 4), (Eighth, 0)));
    vec.push(((B, Flat, 4), (Qtr, 0)));
    vec.push(((A, Natural, 4), (Qtr, 0)));

    vec.push(((F, Natural, 4), (Eighth, 0)));
    vec.push(((F, Natural, 4), (Eighth, 0)));
    vec.push(((B, Flat, 4), (Qtr, 1)));
    vec.push(((C, Natural, 5), (Eighth, 0)));
    vec.push(((D, Natural, 5), (Eighth, 0)));
    vec.push(((E, Flat, 5), (Eighth, 0)));
    vec.push(((F, Natural, 5), (Half, 0)));

    vec.push(((B, Flat, 4), (Eighth, 0)));
    vec.push(((C, Natural, 5), (Eighth, 0)));
    vec.push(((D, Natural, 5), (Qtr, 1)));
    vec.push(((E, Flat, 5), (Eighth, 0)));
    vec.push(((C, Natural, 5), (Qtr, 0)));
    vec.push(((B, Flat, 4), (Whole, 0)));

    return ast::Expr::MusicSeq(vec);
}

pub fn apprasionata() -> ast::Expr {
    let mut vec = Vec::new();

    vec.push(((C, Natural, 3), (Qtr, 0)));
    vec.push(((F, Natural, 3), (Qtr, 0)));
    vec.push(((A, Flat, 3), (Qtr, 0)));
    vec.push(((C, Natural, 4), (Qtr, 0)));
    vec.push(((F, Natural, 4), (Qtr, 0)));
    vec.push(((A, Flat, 4), (Qtr, 1)));
    vec.push(((G, Natural, 4), (Sixteenth, 0)));
    vec.push(((F, Natural, 4), (Sixteenth, 0)));
    vec.push(((E, Natural, 4), (Sixteenth, 0)));
    vec.push(((F, Natural, 4), (Half, 0)));

    vec.push(((G, Natural, 3), (Qtr, 0)));
    vec.push(((C, Natural, 4), (Qtr, 0)));
    vec.push(((E, Natural, 4), (Qtr, 0)));
    vec.push(((G, Natural, 4), (Qtr, 0)));
    vec.push(((B, Flat, 4), (Qtr, 1)));
    vec.push(((A, Flat, 4), (Sixteenth, 0)));
    vec.push(((G, Natural, 4), (Sixteenth, 0)));
    vec.push(((F, Natural, 4), (Sixteenth, 0)));
    vec.push(((G, Natural, 4), (Half, 0)));

    vec.push(((A, Flat, 4), (Qtr, 1)));
    vec.push(((G, Natural, 4), (Sixteenth, 0)));
    vec.push(((F, Natural, 4), (Sixteenth, 0)));
    vec.push(((E, Natural, 4), (Sixteenth, 0)));
    vec.push(((F, Natural, 4), (Half, 0)));

    vec.push(((B, Flat, 4), (Qtr, 1)));
    vec.push(((A, Flat, 4), (Sixteenth, 0)));
    vec.push(((G, Natural, 4), (Sixteenth, 0)));
    vec.push(((F, Natural, 4), (Sixteenth, 0)));
    vec.push(((G, Natural, 4), (Half, 0)));

    vec.push(((C, Natural, 5), (Half, 0)));
    vec.push(((B, Flat, 4), (Eighth, 0)));
    vec.push(((A, Flat, 4), (Eighth, 0)));
    vec.push(((G, Natural, 4), (Eighth, 0)));
    vec.push(((F, Natural, 4), (Eighth, 1)));
    vec.push(((E, Natural, 4), (Sixteenth, 0)));
    vec.push(((F, Natural, 4), (Sixteenth, 0)));
    vec.push(((G, Natural, 4), (Sixteenth, 0)));
    vec.push(((F, Natural, 4), (Qtr, 1)));
    vec.push(((E, Natural, 4), (Half, 0)));

    return ast::Expr::MusicSeq(vec);
}