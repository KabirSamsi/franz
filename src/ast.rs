pub type Handle = String;

#[derive(Clone, PartialEq, Copy, Debug)]
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

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Accidental {
    Sharp,
    Flat,
    Natural,
    Blank
}

#[derive(Clone, Debug)]
pub enum BaseNoteLen {
    Ts,
    Sixteenth,
    Eighth,
    Qtr,
    Half,
    Whole
}

#[derive(Debug)]
pub enum Tempo {
    Lento,
    Adagio,
    Andante,
    Allegro,
    Presto
}

#[derive(Clone, Debug)]
pub enum AExp {
    Int(i32),
    Plus(Box<AExp>, Box<AExp>),
    Times(Box<AExp>, Box<AExp>)
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Param {
    KeySig(Vec<KeySigPitch>),
    Tempo(Tempo),
    TimeSig((AExp, AExp))
}

#[derive(Debug)]
pub enum UntypedExpr {
    Var(Handle),
    Int(i32),   //Done
    Bool(bool), //Done
    Beat(NoteLen),
    Pitch(Pitch),
    Note(Note),
    Motif(Handle, Vec<Handle>, Box<UntypedExpr>),
    Pitches(Handle, Box<UntypedExpr>),
    Phrase(Handle, Box<UntypedExpr>),
    Ternary(Box<UntypedExpr>, Box<UntypedExpr>, Box<UntypedExpr>),
    Plus(Box<UntypedExpr>, Box<UntypedExpr>),
    Seq(Box<UntypedExpr>, Box<UntypedExpr>),
    Times(Box<UntypedExpr>, Box<UntypedExpr>),
    And(Box<UntypedExpr>, Box<UntypedExpr>),
    Or(Box<UntypedExpr>, Box<UntypedExpr>),
    Not(Box<UntypedExpr>),
    Apply(Box<UntypedExpr>, Vec<UntypedExpr>, Box<UntypedExpr>),
    Return(Box<UntypedExpr>),
    Control(Vec<Param>, Vec<UntypedExpr>)
}

#[derive(Debug)]
pub enum RhythmComp {
    Var(Handle),
    Beat(NoteLen),
    Ternary(BExp, Box<RhythmComp>, Box<RhythmComp>),
    Plus(Box<RhythmComp>, Box<RhythmComp>),
    Times(AExp, Box<RhythmComp>)
}

#[derive(Debug)]
pub enum PitchComp {
    Var(Handle),
    Pitch(Pitch),
    Plus(Box<PitchComp>, Box<PitchComp>),
    Times(AExp, Box<PitchComp>)
}

#[derive(Debug)]
pub enum NoteComp {
    Var(Handle),
    Note(Note),
    Plus(Box<NoteComp>, Box<NoteComp>),
    Times(AExp, Box<NoteComp>),
    Apply(RhythmComp, Vec<BExp>, PitchComp)
}

#[derive(Debug)]
pub enum Expr {
    Var(Handle),
    Motif(Vec<Handle>, RhythmComp),
    Pitches(PitchComp),
    Phrase(NoteComp),
    MotifAssgn(Handle, Box<Expr>),
    PitchAssgn(Handle, Box<Expr>),
    PhraseAssgn(Handle, Box<Expr>)
}

pub type Control = (Vec<Param>, Vec<Expr>, Expr);
