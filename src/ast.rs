pub type Handle = String;

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

pub enum AExp {
    Assign(Handle, Box<AExp>),
    Int(i32),
    Plus(Box<AExp>, Box<AExp>),
    Times(Box<AExp>, Box<AExp>)
}

pub enum BExp {
    Assign(Handle, Box<BExp>),
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
    Assign(Handle, Box<RhythmComp>),
    Ternary(BExp, Box<RhythmComp>, Box<RhythmComp>),
    Plus(Box<RhythmComp>, Box<RhythmComp>),
    Times(AExp, Box<RhythmComp>),
    RhythmSequence(Vec<RhythmComp>),
    BeatSequence(Vec<NoteLen>)
}

pub enum PitchComp {
    Var(Handle),
    Pitch(Pitch),
    Ternary(BExp, Box<PitchComp>, Box<PitchComp>),
    Plus(Box<PitchComp>, Box<PitchComp>),
    Times(AExp, Box<PitchComp>),
    PitchCompSeq(Vec<PitchComp>),
    PitchSeq(Vec<Pitch>)
}

pub enum NoteComp {
    Var(Handle),
    Ternary(BExp, Box<NoteComp>, Box<NoteComp>),
    Plus(Box<NoteComp>, Box<NoteComp>),
    Times(AExp, Box<NoteComp>),
    NoteCompSeq(Vec<NoteComp>),
    Phrase(Vec<Note>)
}

pub enum Expr {
    Var(Handle),
    MotifApply(RhythmComp, PitchComp)
}

pub type Control = (Vec<Param>, Vec<Expr>, Expr);
