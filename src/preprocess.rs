use crate::ast::{
    AExp, Accidental, BExp, BaseNoteLen, BasePitch, Handle, KeySigPitch, Note,
    NoteLen, Pitch, PitchComp, RhythmComp
};

fn lookup_motif(_var: Handle) -> RhythmComp {
    RhythmComp::Beat((BaseNoteLen::Qtr, 2))
}

fn lookup_phrase(_var: Handle) -> PitchComp {
    PitchComp::Pitch((BasePitch::C, Accidental::Natural, AExp::Int(4)))
}

// Evaluate an arithmetic expression (simplify before compilation)
fn eval_aexp(e: AExp) -> i32 {
    match e {
        AExp::Var(_) => 1, //TODO
        AExp::Int(i) => i,
        AExp::Plus(e1, e2) => eval_aexp(*e1) + eval_aexp(*e2),
        AExp::Times(e1, e2) => eval_aexp(*e1) * eval_aexp(*e2)
    }
}

// Evaluate an boolean expression (simplify before compilation)
fn eval_bexp(e: BExp) -> bool {
    match e {
        BExp::Var(_) => true, // TODO
        BExp::True => true,
        BExp::False => false,
        BExp::And(b1, b2) => eval_bexp(*b1) && eval_bexp(*b2),
        BExp::Or(b1, b2) => eval_bexp(*b1) || eval_bexp(*b2),
        BExp::Not(b) => !eval_bexp(*b)
    }
}

// Flatten a beat sequence AST to a vector of note/beat lengths
fn flatten_beat(rhythm: RhythmComp) -> Vec<NoteLen> {
    match rhythm {
        RhythmComp::Beat(n) => vec![n],

        RhythmComp::Ternary(b, r1, r2) => {
            if eval_bexp(b) {
                flatten_beat(*r1)
            } else {
                flatten_beat(*r2)
            }
        }
        RhythmComp::Plus(r1, r2) => {
            let (mut v1, v2) = (flatten_beat(*r1), flatten_beat(*r2));
            v1.extend(v2);
            v1
        }
        RhythmComp::Times(e, r) => {
            let n = eval_aexp(e);
            let v = flatten_beat(*r);
            let mut new_vec = Vec::new();
            for _ in 0..n {
                new_vec.extend(v.clone());
            }
            new_vec
        }
        RhythmComp::RhythmSequence(seq) => {
            let mut result_vec = Vec::new();
            for sub_vec in seq {
                result_vec.extend(flatten_beat(sub_vec))
            }
            result_vec
        }
    }
}

// Flatten a pitch sequence AST to a vector of pitches
fn flatten_pitch(pitches: PitchComp) -> Vec<Pitch> {
    match pitches {
        PitchComp::Pitch(n) => vec![n],

        PitchComp::Plus(r1, r2) => {
            let (mut v1, v2) = (flatten_pitch(*r1), flatten_pitch(*r2));
            v1.extend(v2);
            v1
        }
        PitchComp::Times(e, r) => {
            let n = eval_aexp(e);
            let v = flatten_pitch(*r);
            let mut new_vec = Vec::new();
            for _ in 0..n {
                new_vec.extend(v.clone());
            }
            new_vec
        }
        PitchComp::PitchSeq(seq) => {
            let mut result_vec = Vec::new();
            for sub_vec in seq {
                result_vec.extend(flatten_pitch(sub_vec))
            }
            result_vec
        }
    }
}

// Apply a motif to a pitch sequence, thereby generating a series of notes
fn apply_motif(motif: Vec<NoteLen>, pitches: Vec<Pitch>) -> Vec<Note> {
    assert!(motif.len() == pitches.len());
    (1..motif.len())
        .map(|x| (pitches[x].clone(), motif[x].clone()))
        .collect::<Vec<_>>()
}
fn apply_keysig(mut pitch: Pitch, keysig: &Vec<KeySigPitch>) {
    let (base, accidental, octave) = pitch;
    for key in keysig {
        let (keybase, keyacc) = key;
        if (base == *keybase) && (accidental == Accidental::Blank) {
            pitch = (*keybase, *keyacc, octave.clone());
        }
    }
}

fn keysig_phrase(pitches: Vec<Pitch>, keysig: Vec<KeySigPitch>) {
    for pitch in pitches {
        apply_keysig(pitch, &keysig);
    }
}
