// Imports
mod ast;
use crate::ast::*;
use crate::ast::BasePitch::*;
use crate::ast::Accidental::*;
use crate::ast::BaseNoteLen::*;

// Compute a note's MIDI/piano pitch index
fn note_idx(noteval: &BasePitch, acc: &Accidental) -> i32 {
    let note : i32 = match noteval {
        BasePitch::C => 0,
        BasePitch::D => 2,
        BasePitch::E => 4,
        BasePitch::F => 5,
        BasePitch::G => 7,
        BasePitch::A => 9,
        BasePitch::B => 11
    };

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
    let (base, acc, octave, length) = note;
    let idx = 12 * (octave + 1) + note_idx(&base, &acc);
    let diff : f32 = (idx - 69) as f32;
    let freq : f32 = 440.0 * two.powf(diff/12.0);

    // Compute exact length of beat based on note value and dots
    let (beat, dots) = length;
    let mut growth_factor : f32 = 1.0;
    for i in 1..(dots+1) {
        growth_factor += half.powf(i as f32);
    }

    let time : f32 = 0.3 * growth_factor * two.powf(length_idx(&beat) as f32);

    return (freq, time);
}

fn main() {
    let mut vec = Vec::new();

    vec.push((C, Natural, 4, (Eighth, 0)));
    vec.push((D, Natural, 4, (Eighth, 0)));
    vec.push((E, Flat, 4, (Qtr, 1)));
    vec.push((D, Natural, 4, (Eighth, 0)));
    vec.push((E, Flat, 4, (Qtr, 0)));
    vec.push((G, Natural, 4, (Qtr, 0)));
    vec.push((D, Natural, 4, (Half, 1)));

    vec.push((G, Natural, 3, (Qtr, 0)));
    vec.push((C, Natural, 4, (Qtr, 1)));
    vec.push((B, Flat, 3, (Eighth, 0)));
    vec.push((C, Natural, 4, (Qtr, 0)));
    vec.push((E, Flat, 4, (Qtr, 0)));
    vec.push((B, Flat, 3, (Half, 1)));

    println!("SinOsc s => dac;");
    
    let (mut freq, mut time) = (0.0, 0.0);

    for i in 0..vec.len() {
        (freq, time) = process(&vec[i]);
        println!("0.5 => s.gain; {} => s.freq; {} :: second => now;", freq, time);
    }
}