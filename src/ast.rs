pub enum BasePitch {A, B, C, D, E, F, G}
pub enum Accidental{Sharp, Flat, Natural}
pub enum BaseNoteLen{Ts, Sixteenth, Eighth, Qtr, Half, Whole}

// Type synonyms
pub type Dots = i32;
pub type Handle = String;

pub type NoteLen = (BaseNoteLen, Dots);
pub type KeySigPitch = (BasePitch, Accidental);
pub type Pitch = (BasePitch, Accidental, i32);
pub type Note = (Pitch, NoteLen);

pub enum Component {
    KeySig(Vec<(BasePitch, Accidental)>),
    Var(Handle, Box<Component>)
}

pub enum Expr {
    Var(Handle, Box<Expr>),
    Int(i32),
    Bool(bool),
    Motif(Vec<(Handle, Box<Expr>)>, Vec<NoteLen>),
    NoteList(Vec<Note>),
    MotifApply(Box<Expr>, Box<Expr>),
    MusicSeq(Vec<(NoteLen, Note)>),
    Plus(Box<Expr>, Box<Expr>),
    Times(Box<Expr>, Box<Expr>),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>)
}

pub type Control = (Vec<Component>, Vec<Expr>, Expr);