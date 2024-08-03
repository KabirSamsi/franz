use crate::ast;
use crate::ast::*;
use crate::ast::AExp::*;
use crate::ast::BExp::*;
use crate::ast::RhythmComp::*;
use crate::ast::NoteComp::*;

fn lookupRhythm(name: &crate::Handle) -> &crate::RhythmComp {
    // TODO
    return RhythmComp::Handle("Todo");
}

fn lookupBExp(name : &crate::Handle) -> &crate::BExp {
    //TODO 
    return BExp::True;
}

// Lookup motif based on handle
fn lookupMotif(name : &crate::Handle) -> (Vec<NoteLen>, Vec<Pitch>) {
    // TODO
    return (Vec::new(), Vec::new());
}

// Evaluate a boolean expression to a boolean
fn evalBExp(exp : &crate::BExp) -> bool {
    return match e {
        True => true,
        False => false,
        Var(handle) => evalBExp(lookupBExp(handle)),
        Not(e) => !(evalBExp(e)),
        And(e1, e2) => evalBExp(e1) & evalBExp(e2),
        Or(e1, e2) => evalBExp(e1) | evalBExp(e2),
    }
}

// Evaluate an arithmetic expression to an integer
fn evalAExp(exp : &crate::AExp) -> i32 {
    return match e {
        Int(n) => n,
        Var(handle) => evalAExp(lookupAExp(handle)),
        Plus(e1, e2) => evalAExp(e1) + evalAExp(e2),
        Times(e1, e2) => evalAExp(e1) * evalAExp(e2)
    }
}

// Transform rhythm component into just a series of beats
fn flattenRhythm(rhythm: &crate::RhythmComp) -> Vec<NoteLen> {
    return match rhythm {
        Var(handle) => lookupRhythm(handle), //Variable
        Beat(beat) => vec![beat], //Single Beat

        Plus(comp1, comp2) => { //Add two rhythm sequences together
            Beat(!vec[
                flattenRhythm(comp1),
                flattenRhythm(comp2)].concat()
            )
        },

        Times(n, comp) => { //Multiply a rhythm sequence n times
            let mut result = flattenRhythm(comp);
            let extension = result.clone();
            for i in 0..n {result.extend(extension);}
            result
        },

        Ternary(b, comp1, comp2) => { //Ternary sequence to determine which sequence to hold
            if evalBExp(b) {
                flattenRhythm(comp1);
            } else {
                flattenRhythm(comp2);
            }
        },

        BeatSequence(components) { //Flatten a collection of rhythm rules into one sequence
            result : Vec<NoteLen> = Vec::new();
            for i in 0..components.len() {
                result.extend(flattenRhythm(components[i]));
            }
            result
        }
    }
}

// Transform pitch component into a series of pitches
fn flattenPitches(pitch: &crate::PitchComp) -> Vec<Pitch> {
    // TODO
    return Vec::new();
}

// Transform note component into a series of notes
fn flattenNoteSeq(pitch: &crate::NoteComp) -> Vec<Note> {
    // TODO
    return Vec::new();
}

// Apply a rhythm sequence to a pitch sequence to generate a note phrase
pub fn applyRhythm(phrase: &crate::Expr) -> &crate::NoteComp {
    let (mut rhythm, mut pitches) = match phrase {
        MotifApply(r, p) => (r, p),
        Var(handle) => lookupMotif(handle)
    };

    let beats : Vec<NoteLen> = flattenRhythm(rhythm);
    let pitches : Vec<Pitch> = flattenPitches(pitches);
    assert!(beats.len() == pitches.len());

    let notes : Vec<Note> = Vec::new();
    for i in 0..beats.len() {
        notes.push((beats[i], pitches[i]));
    }

    return NoteComp::Phrase(notes);
}