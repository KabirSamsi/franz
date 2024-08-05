use crate::ast::{
    AExp::{self, *},
    BExp::{self, *},
    Expr, NoteComp, NoteLen, Pitch, PitchComp,
    RhythmComp::{self, *}
};

fn lookup_bexp(name: crate::ast::Handle) -> crate::ast::BExp {
    //TODO
    BExp::True
}

fn lookup_aexp(name: crate::ast::Handle) -> crate::ast::AExp {
    //TODO
    AExp::Int(0)
}

// Lookup motif based on handle
fn lookup_motif(name: crate::ast::Handle) -> (Vec<NoteLen>) {
    // TODO
    Vec::new()
}

fn lookup_rhythm(name: crate::ast::Handle) -> crate::ast::RhythmComp {
    // TODO
    RhythmComp::Var("Todo".to_string())
}

// Evaluate a boolean expression to a boolean
fn eval_bexp(exp: &crate::ast::BExp) -> bool {
    match exp {
        True => true,
        False => false,
        BExp::Var(handle) => eval_bexp(lookup_bexp(handle)),
        Not(e) => !(eval_bexp(e)),
        And(e1, e2) => eval_bexp(e1) & eval_bexp(e2),
        Or(e1, e2) => eval_bexp(e1) | eval_bexp(e2)
    }
}

// Evaluate an arithmetic expression to an integer
fn eval_aexp(exp: &crate::ast::AExp) -> i32 {
    match exp {
        Int(n) => *n,
        AExp::Var(handle) => eval_aexp(lookup_aexp(handle)),
        AExp::Plus(e1, e2) => eval_aexp(e1) + eval_aexp(e2),
        AExp::Times(e1, e2) => eval_aexp(e1) * eval_aexp(e2)
    }
}

// Transform rhythm component into just a series of beats
fn flatten_rhythm(rhythm: crate::ast::RhythmComp) -> Vec<NoteLen> {
    match rhythm {
        BeatSequence(beats) => beats,
        Beat(beat) => vec![beat], //Single Beat

        RhythmComp::Var(handle) => {
            /* Variable */
            flatten_rhythm(lookup_rhythm(&handle))
        }

        RhythmComp::Plus(comp1, comp2) => {
            //Add two rhythm sequences together
            let sum = flatten_rhythm(*comp1);
            sum.extend(flatten_rhythm(*comp2));
            sum
        }

        RhythmComp::Times(n, comp) => {
            //Multiply a rhythm sequence n times
            let mut result = flatten_rhythm(*comp);
            let extension = result.clone();
            for i in 0..eval_aexp(&n) {
                result.extend(extension);
            }
            result
        }

        RhythmComp::Ternary(b, comp1, comp2) => {
            //Ternary sequence to determine which sequence to hold
            if eval_bexp(&b) {
                flatten_rhythm(*comp1)
            } else {
                flatten_rhythm(*comp2)
            }
        }

        RhythmComp::RhythmSequence(components) => {
            //Flatten a collection of rhythm rules into one sequence
            let result: Vec<NoteLen> = Vec::new();
            for i in 0..components.len() {
                result.extend(flatten_rhythm(components[i]));
            }
            result
        }
    }
}

// Transform pitch component into a series of pitches
fn flatten_pitches(pitch: crate::ast::PitchComp) -> Vec<Pitch> {
    // TODO
    Vec::new()
}

// Transform note component into a series of notes
fn flatten_note_seq(pitch: crate::ast::NoteComp) -> Vec<crate::ast::Note> {
    // TODO
    Vec::new()
}

// Apply a rhythm sequence to a pitch sequence to generate a note phrase
pub fn apply_rhythm(phrase: &crate::ast::Expr) -> &crate::ast::NoteComp {
    let (rhythm, pitches) = match phrase {
        Expr::MotifApply(r, p) => (r, p),
        Expr::Var(handle) => {
            let rhythm = RhythmComp::Var(handle.clone());
            let pitches = PitchComp::Var(handle.clone());
            (&rhythm, &pitches)
        }
    };

    let beats: Vec<NoteLen> = flatten_rhythm(*rhythm);
    let pitches: Vec<Pitch> = flatten_pitches(*pitches);
    assert!(beats.len() == pitches.len());

    let notes: Vec<crate::ast::Note> = Vec::new();
    for i in 0..beats.len() {
        notes.push((pitches[i], beats[i]));
    }

    &NoteComp::Phrase(notes)
}
