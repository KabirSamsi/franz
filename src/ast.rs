pub type Handle = String;

#[derive(Clone, PartialEq, Copy)]
pub enum BasePitch {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    Rest
}

#[derive(Clone, PartialEq, Copy)]
pub enum Accidental {
    Sharp,
    Flat,
    Natural,
    Blank
}

#[derive(Clone)]
pub enum BaseNoteLen {
    Ts,
    Sixteenth,
    Eighth,
    Qtr,
    Half,
    Whole
}

pub enum Tempo {
    Lento,
    Adagio,
    Andante,
    Allegro,
    Presto
}

#[derive(Clone)]
pub enum AExp {
    Var(Handle),
    Int(i32),
    Plus(Box<AExp>, Box<AExp>),
    Times(Box<AExp>, Box<AExp>)
}

pub enum BExp {
    Var(Handle),
    True,
    False,
    And(Box<BExp>, Box<BExp>),
    Or(Box<BExp>, Box<BExp>),
    Not(Box<BExp>)
}

// Type synonyms
pub type Dots = i32;
pub type NoteLen = (BaseNoteLen, Dots);
pub type KeySigPitch = (BasePitch, Accidental);
pub type Pitch = (BasePitch, Accidental, AExp);
pub type Note = (Pitch, NoteLen);

pub enum Param {
    KeySig(Vec<KeySigPitch>),
    Tempo(Tempo),
    TimeSig((AExp, AExp))
}

pub enum RhythmComp {
    Beat(NoteLen),
    Ternary(BExp, Box<RhythmComp>, Box<RhythmComp>),
    Plus(Box<RhythmComp>, Box<RhythmComp>),
    Times(AExp, Box<RhythmComp>),
    RhythmSequence(Vec<RhythmComp>)
}

pub enum PitchComp {
    Pitch(Pitch),
    Plus(Box<PitchComp>, Box<PitchComp>),
    Times(AExp, Box<PitchComp>),
    PitchSeq(Vec<PitchComp>)
}

pub enum NoteComp {
    Var(Handle),
    Note(Note),
    Plus(Box<NoteComp>, Box<NoteComp>),
    Times(AExp, Box<NoteComp>),
    Phrase(Vec<NoteComp>)
}

pub enum Expr {
    Motif(Vec<Handle>, RhythmComp),
    Phrase(Vec<Handle>, PitchComp),
    MotifAssgn(Handle, RhythmComp),
    PitchAssgn(Handle, PitchComp),
    NoteAssgn(Handle, PitchComp),
    MotifApply(RhythmComp, PitchComp)
}

pub type Control = (Vec<Param>, Vec<Expr>, Expr);
