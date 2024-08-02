// Imports
mod ast;
use std::fs::File;
use std::io::prelude::*;
use crate::ast::*;
use crate::ast::BasePitch::*;
use crate::ast::Accidental::*;
use crate::ast::BaseNoteLen::*;

// Compute a note's MIDI/piano pitch index
fn note_idx(noteval: &BasePitch, acc: &Accidental) -> i32 {
    // Compute note index based on pitch
    let note : i32 = match noteval {
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
fn process(note : &Note) -> (f32, f32) {
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
    let time : f32 = (300.0 * growth_factor * two.powf(length_idx(&beat) as f32)).round() /1000.0;

    return (freq, time);
}

fn main() -> std::io::Result<()> {
    // Write results directly to file
    let mut file = File::create("chuck-programs/innocent.ck")?;
    let mut vec = Vec::new();

    // For now, just an IR representation of what `franz-programs/innocent.ck` should look like at the beginning!
    vec.push(((C, Natural, 4), (Eighth, 0)));
    vec.push(((D, Natural, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Qtr, 1)));
    vec.push(((D, Natural, 4), (Eighth, 0)));
    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((G, Natural, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Half, 1)));

    vec.push(((G, Natural, 3), (Qtr, 0)));
    vec.push(((C, Natural, 4), (Qtr, 1)));
    vec.push(((B, Flat, 3), (Eighth, 0)));
    vec.push(((C, Natural, 4), (Qtr, 0)));
    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((B, Flat, 3), (Half, 1)));

    vec.push(((G, Natural, 3), (Qtr, 0)));
    vec.push(((A, Flat, 3), (Qtr, 1)));
    vec.push(((G, Natural, 3), (Eighth, 0)));
    vec.push(((A, Flat, 3), (Qtr, 0)));
    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((G, Natural, 3), (Half, 1)));

    vec.push(((E, Flat, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Qtr, 1)));
    vec.push(((A, Natural, 3), (Eighth, 0)));
    vec.push(((A, Natural, 3), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Qtr, 0)));
    vec.push(((D, Natural, 4), (Half, 1)));

    
    let (mut freq, mut time) = (0.0, 0.0);

    file.write_all((b"SinOsc s => dac;\n"));

    for i in 0..vec.len() { //Process each line and write it
        (freq, time) = process(&vec[i]);
        let _ = file.write_all((
            format!("0.5 => s.gain; {freq} => s.freq; {time} :: second => now;\n")
        ).as_bytes())?;
    }
    return Ok(());
}