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
pub enum Accidental {
    Sharp,
    Flat,
    Natural,
    Blank
}
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

// Type synonyms
pub type Dots = i32;
pub type Handle = String;

pub type NoteLen = (BaseNoteLen, Dots);
pub type KeySigPitch = (BasePitch, Accidental);
pub type Pitch = (BasePitch, Accidental, i32);
pub type Note = (Pitch, NoteLen);

pub enum Param {
    Var(Handle),
    KeySig(Vec<KeySigPitch>),
    Tempo(Tempo),
    TimeSig((i32, i32))
}

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

pub enum RhythmComp {
    Var(Handle),
    Beat(NoteLen),
    Ternary(BExp, Box<RhythmComp>, Box<RhythmComp>),
    Plus(Box<RhythmComp>, Box<RhythmComp>),
    Times(AExp, Box<RhythmComp>),
    BeatSequence(Vec<RhythmComp>)
}

pub enum PitchComp {
    Var(Handle),
    Pitch(Pitch),
    Ternary(BExp, Box<PitchComp>, Box<PitchComp>),
    Plus(Box<PitchComp>, Box<PitchComp>),
    Times(AExp, Box<PitchComp>),
    PitchSeq(Vec<PitchComp>)
}

pub enum NoteComp {
    Var(Handle),
    Note(Note),
    Ternary(BExp, Box<NoteComp>, Box<NoteComp>),
    Plus(Box<NoteComp>, Box<NoteComp>),
    Times(AExp, Box<NoteComp>),
    Phrase(Vec<NoteComp>)
}

pub enum Expr {
    Var(Handle),
    MotifApply(RhythmComp, NoteComp)
}

pub type Control = (Vec<Param>, Vec<Expr>, Expr);
