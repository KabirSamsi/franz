use crate::ast::{AExp, BExp, NoteComp, PitchComp, RhythmComp, UntypedExpr};

use crate::error::{FranzError, FranzResult};

fn expr_to_aexp(expr: &UntypedExpr) -> FranzResult<AExp> {
    match expr {
        UntypedExpr::Int(i) => Ok(AExp::Int(*i)),
        UntypedExpr::Plus(e1, e2) => {
            let (a1, a2) = (expr_to_aexp(e1), expr_to_aexp(e2));
            Ok(AExp::Plus(Box::new(a1?), Box::new(a2?)))
        }

        UntypedExpr::Times(e1, e2) => {
            let (a1, a2) = (expr_to_aexp(e1), expr_to_aexp(e2));
            Ok(AExp::Times(Box::new(a1?), Box::new(a2?)))
        }
        _ => Err(FranzError::FlattenError)
    }
}

fn expr_to_bexp(expr: &UntypedExpr) -> FranzResult<BExp> {
    match expr {
        //VAR RULE
        UntypedExpr::Bool(true) => Ok(BExp::True),
        UntypedExpr::Bool(false) => Ok(BExp::False),
        UntypedExpr::Not(e) => Ok(BExp::Not(Box::new(expr_to_bexp(e)?))),
        UntypedExpr::And(e1, e2) => Ok(BExp::And(
            Box::new(expr_to_bexp(e1)?),
            Box::new(expr_to_bexp(e2)?)
        )),
        UntypedExpr::Or(e1, e2) => Ok(BExp::Or(
            Box::new(expr_to_bexp(e1)?),
            Box::new(expr_to_bexp(e2)?)
        )),
        _ => Err(FranzError::FlattenError)
    }
}

fn expr_to_rhythmcomp(expr: &UntypedExpr) -> FranzResult<RhythmComp> {
    match expr {
        //VAR RULE
        UntypedExpr::Beat(l) => Ok(RhythmComp::Beat(l.clone())),
        UntypedExpr::Ternary(b, e1, e2) => Ok(RhythmComp::Ternary(
            expr_to_bexp(b)?,
            Box::new(expr_to_rhythmcomp(e1)?),
            Box::new(expr_to_rhythmcomp(e2)?)
        )),

        UntypedExpr::Plus(e1, e2) => Ok(RhythmComp::Plus(
            Box::new(expr_to_rhythmcomp(e1)?),
            Box::new(expr_to_rhythmcomp(e2)?)
        )),

        UntypedExpr::Times(e1, e2) => Ok(RhythmComp::Times(
            expr_to_aexp(e1)?,
            Box::new(expr_to_rhythmcomp(e2)?)
        )),
        _ => Err(FranzError::FlattenError)
    }
}

fn expr_to_pitchcomp(expr: &UntypedExpr) -> FranzResult<PitchComp> {
    match expr {
        //VAR RULE
        UntypedExpr::Pitch(p) => Ok(PitchComp::Pitch(p.clone())),

        UntypedExpr::Plus(e1, e2) => Ok(PitchComp::Plus(
            Box::new(expr_to_pitchcomp(e1)?),
            Box::new(expr_to_pitchcomp(e2)?)
        )),

        UntypedExpr::Times(e1, e2) => Ok(PitchComp::Times(
            expr_to_aexp(e1)?,
            Box::new(expr_to_pitchcomp(e2)?)
        )),
        _ => Err(FranzError::FlattenError)
    }
}

fn expr_to_notecomp(expr: &UntypedExpr) -> FranzResult<NoteComp> {
    match expr {
        //VAR RULE
        UntypedExpr::Note(n) => Ok(NoteComp::Note(n.clone())),

        UntypedExpr::Plus(e1, e2) => Ok(NoteComp::Plus(
            Box::new(expr_to_notecomp(e1)?),
            Box::new(expr_to_notecomp(e2)?)
        )),

        UntypedExpr::Times(e1, e2) => Ok(NoteComp::Times(
            expr_to_aexp(e1)?,
            Box::new(expr_to_notecomp(e2)?)
        )),

        UntypedExpr::Apply(e1, args, e2) => Ok(NoteComp::Apply(
            expr_to_rhythmcomp(e1)?,
            args.iter()
                .map(expr_to_bexp)
                .collect::<Result<Vec<BExp>, FranzError>>()?,
            expr_to_pitchcomp(e2)?
        )),

        _ => Err(FranzError::FlattenError)
    }
}
