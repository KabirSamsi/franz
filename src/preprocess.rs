use crate::ast;
use crate::ast::*;
use crate::ast::AExp::*;
use crate::ast::BExp::*;
use crate::ast::RhythmComp::*;
use crate::ast::NoteComp::*;

// Lookup motif based on handle
fn lookupMotif(name : &crate::Handle) -> (Vec<NoteLen>, Vec<Pitch>) {
    // TODO
    return (Vec::new(), Vec::new());
}

// Transform rhythm component into just a series of beats
fn flattenRhythm(rhythm: &crate::RhythmComp) -> Vec<NoteLen> {
    // TODO
    return Vec::new();
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