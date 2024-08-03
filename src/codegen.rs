use std::fs::File;
use std::io::Write;
use crate::ast::*;
use crate::ast::NoteComp;

// Compute a note's MIDI/piano pitch index
fn note_idx(noteval: &BasePitch, acc: &Accidental) -> i32 {
    // Compute note index based on pitch
    let note : i32 = match noteval {
        BasePitch::Rest => -1,
        BasePitch::C => 0,
        BasePitch::D => 2,
        BasePitch::E => 4,
        BasePitch::F => 5,
        BasePitch::G => 7,
        BasePitch::A => 9,
        BasePitch::B => 11
    };

    // Compute offset based on accidental
    let accidental : i32 = match acc {
        Accidental::Flat => -1,
        Accidental::Natural => 0,
        Accidental::Blank => 0,
        Accidental::Sharp => 1
    };

    return note + accidental;
}

// Compute a beat length's 'index'
fn length_idx(beatval: &BaseNoteLen) -> i32 {
    return match beatval {
        BaseNoteLen::Ts => -2,
        BaseNoteLen::Sixteenth => -1,
        BaseNoteLen::Eighth => 0,
        BaseNoteLen::Qtr => 1,
        BaseNoteLen::Half => 2,
        BaseNoteLen::Whole => 3
    };
}

// Compute frequency and length of a note
fn process(note : &Note, speed : f32) -> (f32, f32) {
    let two : f32 = 2.0;
    let half : f32 = 0.5;
    
    // Match out note components. Compute frequency based on this.
    let (pitch, length) = note;
    let (base, acc, octave) = pitch;

    let idx = 12 * (octave + 1) + note_idx(&base, &acc);
    let diff : f32 = (idx - 69) as f32;
    let freq : f32 = 440.0 * two.powf(diff/12.0);

    // Compute exact length of beat based on note value and dots
    let (beat, dots) = length;
    let mut growth_factor : f32 = 1.0;
    for i in 1..(dots+1) {
        growth_factor += half.powf(i as f32);
    }

    // Truncated floating-point precision calculation for time
    let time : f32 = (1000.0 * speed * growth_factor * two.powf(length_idx(&beat) as f32)).round() /1000.0;

    return (freq, time);
}

// Compile an AST to a series of notes, and write to file
pub fn compile_seq(name : &str, phrase : NoteComp, speed : f32, print : bool) -> std::io::Result<()> {
    //Write results to file
    let mut file = File::create(format!("chuck-programs/{name}.ck"))?;
    let (mut freq, mut time);

    let empty = Vec::new();

    let notes = match phrase { //Extract out note sequence, if present
        NoteComp::Phrase(v) => v,
        _ => empty
    };

    let _ = file.write_all(b"SinOsc s => dac;\n");
    
    if print {
        let _ = println!("SinOsc s => dac;");
    }

    for i in 0..notes.len() { //Process each line and write it
        (freq, time) = process(&notes[i], speed);
        let _ = file.write_all((
            format!("0.5 => s.gain; {freq} => s.freq; {time} :: second => now;\n")
        ).as_bytes())?;
        if print {
            let _ = println!("0.5 => s.gain; {freq} => s.freq; {time} :: second => now;");
        }
    }
    return Ok(());
}