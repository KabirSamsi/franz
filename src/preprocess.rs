use crate::ast::{
    AExp, Accidental, BExp, Handle, KeySigPitch, Note, NoteComp, NoteLen,
    Pitch, PitchComp, RhythmComp
};

use crate::error::{FranzError, FranzResult};

use std::collections::HashMap;

//Lookup pre-defined sequences
macro_rules! define_lookup {
    ($name: ident, $storetype:ident) => {
        fn $name(
            var: Handle, store: &mut HashMap<String, $storetype>
        ) -> FranzResult<&$storetype> {
            match store.get(&var) {
                Some(v) => Ok(v),
                None => Err(FranzError::UnboundError)
            }
        }
    };
}

define_lookup!(lookup_motif, RhythmComp);
define_lookup!(lookup_pitches, PitchComp);
define_lookup!(lookup_phrase, NoteComp);

// Evaluate an arithmetic expression (simplify before compilation)
fn eval_aexp(e: &AExp) -> i32 {
    match e {
        AExp::Int(i) => *i,
        AExp::Plus(e1, e2) => eval_aexp(e1) + eval_aexp(e2),
        AExp::Times(e1, e2) => eval_aexp(e1) * eval_aexp(e2)
    }
}

// Evaluate an boolean expression (simplify before compilation)
fn eval_bexp(e: &BExp) -> bool {
    match e {
        BExp::Var(_) => true, // TODO
        BExp::True => true,
        BExp::False => false,
        BExp::And(b1, b2) => eval_bexp(b1) && eval_bexp(b2),
        BExp::Or(b1, b2) => eval_bexp(b1) || eval_bexp(b2),
        BExp::Not(b) => !eval_bexp(b)
    }
}

// Flatten a beat sequence AST to a vector of note/beat lengths
pub fn flatten_beat(rhythm: &RhythmComp) -> Vec<NoteLen> {
    match rhythm {
        RhythmComp::Var(_) => vec![],
        RhythmComp::Beat(n) => vec![n.clone()],

        RhythmComp::Ternary(b, r1, r2) => {
            if eval_bexp(b) {
                flatten_beat(r1)
            } else {
                flatten_beat(r2)
            }
        }
        RhythmComp::Plus(r1, r2) => {
            let (mut v1, v2) = (flatten_beat(r1), flatten_beat(r2));
            v1.extend(v2);
            v1
        }
        RhythmComp::Times(e, r) => {
            let n = eval_aexp(e);
            let v = flatten_beat(r);
            let mut new_vec = Vec::new();
            for _ in 0..n {
                new_vec.extend(v.clone());
            }
            new_vec
        }
    }
}

// Flatten a pitch sequence AST to a vector of pitches
pub fn flatten_pitch(pitches: &PitchComp) -> Vec<Pitch> {
    match pitches {
        PitchComp::Var(_) => vec![],
        PitchComp::Pitch(n) => vec![*n],

        PitchComp::Plus(r1, r2) => {
            let (mut v1, v2) = (flatten_pitch(r1), flatten_pitch(r2));
            v1.extend(v2);
            v1
        }
        PitchComp::Times(e, r) => {
            let n = eval_aexp(e);
            let v = flatten_pitch(r);
            let mut new_vec = Vec::new();
            for _ in 0..n {
                new_vec.extend(v.clone());
            }
            new_vec
        }
    }
}

// Apply a motif to a pitch sequence, thereby generating a series of notes
pub fn apply_motif(motif: Vec<NoteLen>, pitches: Vec<Pitch>) -> Vec<Note> {
    assert!(motif.len() == pitches.len());
    (0..motif.len())
        .map(|x| (pitches[x], motif[x].clone()))
        .collect::<Vec<_>>()
}

// Apply a key signature to a note
fn apply_keysig(mut pitch: Pitch, keysig: &Vec<KeySigPitch>) -> Pitch {
    let (base, accidental, octave) = pitch;
    for key in keysig {
        let (keybase, keyacc) = key;
        if base == *keybase && accidental == Accidental::Blank {
            pitch = (base, *keyacc, octave);
        }
    }
    pitch
}

// Apply a key signature to a sequence of notes
fn keysig_phrase(mut pitches: Vec<Pitch>, keysig: Vec<KeySigPitch>) {
    for pitch in &mut pitches {
        *pitch = apply_keysig(*pitch, &keysig);
    }
}
