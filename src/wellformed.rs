use crate::ast::{
    AExp, BExp, Control, Expr, NoteComp, PitchComp, RhythmComp, UntypedExpr,
    VarType
};

use crate::error::{FranzError, FranzResult};

use std::collections::HashMap;

// Obtain the type of a variable
fn type_var(
    keyword: &String, map: &mut HashMap<String, VarType>
) -> FranzResult<VarType> {
    match map.get(keyword) {
        Some(var) => Ok(*var),
        None => Err(FranzError::UnboundError)
    }
}

//Compile untyped expression to arithmetic expression
fn uexpr_to_aexp(expr: &UntypedExpr) -> FranzResult<AExp> {
    match expr {
        UntypedExpr::Int(i) => Ok(AExp::Int(*i)),
        UntypedExpr::Plus(e1, e2) => {
            let (a1, a2) = (uexpr_to_aexp(e1), uexpr_to_aexp(e2));
            Ok(AExp::Plus(Box::new(a1?), Box::new(a2?)))
        }

        UntypedExpr::Times(e1, e2) => {
            let (a1, a2) = (uexpr_to_aexp(e1), uexpr_to_aexp(e2));
            Ok(AExp::Times(Box::new(a1?), Box::new(a2?)))
        }
        _ => Err(FranzError::ParseError)
    }
}

//Compile untyped expression to boolean expression
fn uexpr_to_bexp(
    argmap: &mut HashMap<String, VarType>, expr: &UntypedExpr
) -> FranzResult<BExp> {
    match expr {
        //Check that variable is correctly typed as boolean
        UntypedExpr::Var(handle) => match type_var(handle, argmap) {
            Ok(data) => match data {
                VarType::Bool => Ok(BExp::Var(handle.to_string())),
                _ => Err(FranzError::ParseError)
            },
            _ => Err(FranzError::UnboundError)
        },

        UntypedExpr::Bool(true) => Ok(BExp::True),
        UntypedExpr::Bool(false) => Ok(BExp::False),
        UntypedExpr::Not(e) => {
            Ok(BExp::Not(Box::new(uexpr_to_bexp(argmap, e)?)))
        }
        UntypedExpr::And(e1, e2) => Ok(BExp::And(
            Box::new(uexpr_to_bexp(argmap, e1)?),
            Box::new(uexpr_to_bexp(argmap, e2)?)
        )),
        UntypedExpr::Or(e1, e2) => Ok(BExp::Or(
            Box::new(uexpr_to_bexp(argmap, e1)?),
            Box::new(uexpr_to_bexp(argmap, e2)?)
        )),
        _ => Err(FranzError::ParseError)
    }
}

//Compile untyped expression to rhythm component
fn uexpr_to_rhythmcomp(
    argmap: &mut HashMap<String, VarType>, expr: &UntypedExpr
) -> FranzResult<RhythmComp> {
    match expr {
        //Check that variable is correctly typed as motif
        UntypedExpr::Var(handle) => match type_var(handle, argmap) {
            Ok(data) => match data {
                VarType::Motif => Ok(RhythmComp::Var(handle.to_string())),
                _ => Err(FranzError::ParseError)
            },
            _ => Err(FranzError::UnboundError)
        },

        UntypedExpr::Beat(l) => Ok(RhythmComp::Beat(l.clone())),
        UntypedExpr::Ternary(b, e1, e2) => Ok(RhythmComp::Ternary(
            uexpr_to_bexp(argmap, b)?,
            Box::new(uexpr_to_rhythmcomp(argmap, e1)?),
            Box::new(uexpr_to_rhythmcomp(argmap, e2)?)
        )),

        UntypedExpr::Plus(e1, e2) => Ok(RhythmComp::Plus(
            Box::new(uexpr_to_rhythmcomp(argmap, e1)?),
            Box::new(uexpr_to_rhythmcomp(argmap, e2)?)
        )),

        UntypedExpr::Times(e1, e2) => Ok(RhythmComp::Times(
            uexpr_to_aexp(e1)?,
            Box::new(uexpr_to_rhythmcomp(argmap, e2)?)
        )),
        _ => Err(FranzError::ParseError)
    }
}

//Compile untyped expression to pitch component
fn uexpr_to_pitchcomp(
    argmap: &mut HashMap<String, VarType>, expr: &UntypedExpr
) -> FranzResult<PitchComp> {
    match expr {
        //Check that variable is correctly typed as pitch sequence
        UntypedExpr::Var(handle) => match type_var(handle, argmap) {
            Ok(data) => match data {
                VarType::Pitches => Ok(PitchComp::Var(handle.to_string())),
                _ => Err(FranzError::ParseError)
            },
            _ => Err(FranzError::UnboundError)
        },

        UntypedExpr::Pitch(p) => Ok(PitchComp::Pitch(p.clone())),

        UntypedExpr::Plus(e1, e2) => Ok(PitchComp::Plus(
            Box::new(uexpr_to_pitchcomp(argmap, e1)?),
            Box::new(uexpr_to_pitchcomp(argmap, e2)?)
        )),

        UntypedExpr::Times(e1, e2) => Ok(PitchComp::Times(
            uexpr_to_aexp(e1)?,
            Box::new(uexpr_to_pitchcomp(argmap, e2)?)
        )),
        _ => Err(FranzError::ParseError)
    }
}

//Compile untyped expression to note component
fn uexpr_to_notecomp(
    argmap: &mut HashMap<String, VarType>, expr: &UntypedExpr
) -> FranzResult<NoteComp> {
    match expr {
        //Check that variable is correctly typed as phrase
        UntypedExpr::Var(handle) => match type_var(handle, argmap) {
            Ok(data) => match data {
                VarType::Phrase => Ok(NoteComp::Var(handle.to_string())),
                _ => Err(FranzError::ParseError)
            },
            _ => Err(FranzError::UnboundError)
        },

        UntypedExpr::Note(n) => Ok(NoteComp::Note(n.clone())),

        UntypedExpr::Plus(e1, e2) => Ok(NoteComp::Plus(
            Box::new(uexpr_to_notecomp(argmap, e1)?),
            Box::new(uexpr_to_notecomp(argmap, e2)?)
        )),

        UntypedExpr::Times(e1, e2) => Ok(NoteComp::Times(
            uexpr_to_aexp(e1)?,
            Box::new(uexpr_to_notecomp(argmap, e2)?)
        )),

        UntypedExpr::Apply(e1, args, e2) => {
            let mut result = Vec::new();
            for arg in args {
                result.push(uexpr_to_bexp(argmap, arg)?);
            }

            Ok(NoteComp::Apply(
                uexpr_to_rhythmcomp(argmap, e1)?,
                result,
                uexpr_to_pitchcomp(argmap, e2)?
            ))
        }

        _ => Err(FranzError::ParseError)
    }
}

//Compile untyped expression to well-formed Franz expression
fn uexpr_to_expr(
    argmap: &mut HashMap<String, VarType>, expr: &UntypedExpr
) -> FranzResult<Expr> {
    match expr {
        UntypedExpr::Motif(name, args, comp) => {
            //Bind argument names as new variables
            for arg in args {
                argmap.insert(arg.to_string(), VarType::Bool);
            }

            //Bind motif as new variable
            let result = Expr::MotifAssgn(
                name.to_string(),
                Box::new(Expr::Motif(
                    args.to_vec(),
                    uexpr_to_rhythmcomp(argmap, comp)?
                ))
            );
            argmap.insert(name.to_string(), VarType::Motif);
            Ok(result)
        }

        UntypedExpr::Pitches(name, comp) => {
            let result = Expr::PitchAssgn(
                name.to_string(),
                Box::new(Expr::Pitches(uexpr_to_pitchcomp(argmap, comp)?))
            );
            //Bind pitch sequence as new variable
            argmap.insert(name.to_string(), VarType::Pitches);
            Ok(result)
        }

        UntypedExpr::Phrase(name, comp) => {
            let result = Expr::PhraseAssgn(
                name.to_string(),
                Box::new(Expr::Phrase(uexpr_to_notecomp(argmap, comp)?))
            );
            //Bind phrase as new variable
            argmap.insert(name.to_string(), VarType::Phrase);
            Ok(result)
        }

        _ => Err(FranzError::ParseError)
    }
}

//Compile untyped expression to control sequence
pub fn uexpr_to_control(
    argmap: &mut HashMap<String, VarType>, expr: &UntypedExpr
) -> FranzResult<Control> {
    match expr {
        UntypedExpr::Control(params, comps) => {
            if comps.is_empty() {
                Err(FranzError::ParseError)
            } else {
                //Parse return separately from central components
                let (hd, tl) =
                    (&comps[..comps.len() - 1], &comps[comps.len() - 1]);

                match tl {
                    UntypedExpr::Return(e) => {
                        let mut mapped_head = Vec::new();
                        for elem in hd {
                            mapped_head.push(uexpr_to_expr(argmap, elem)?);
                        }
                        Ok((
                            params.to_vec(),
                            mapped_head,
                            Expr::Phrase(uexpr_to_notecomp(argmap, e)?)
                        ))
                    }
                    _ => Err(FranzError::ParseError)
                }
            }
        }
        _ => Err(FranzError::ParseError)
    }
}
