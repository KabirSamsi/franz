use crate::{
    ast::{Accidental, BaseNoteLen, BasePitch, NoteComp},
    error::{FranzError, FranzResult}
};
use std::{fs::File, io::Write};

// Compute a note's MIDI/piano pitch index
fn note_idx(noteval: &BasePitch, acc: &Accidental) -> i32 {
    // Compute note index based on pitch
    let note: i32 = match noteval {
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
    let accidental: i32 = match acc {
        Accidental::Flat => -1,
        Accidental::Natural => 0,
        Accidental::Blank => 0,
        Accidental::Sharp => 1
    };

    note + accidental
}

// Compute a beat length's 'index'
fn length_idx(beatval: &BaseNoteLen) -> i32 {
    match beatval {
        BaseNoteLen::Ts => -2,
        BaseNoteLen::Sixteenth => -1,
        BaseNoteLen::Eighth => 0,
        BaseNoteLen::Qtr => 1,
        BaseNoteLen::Half => 2,
        BaseNoteLen::Whole => 3
    }
}

// Compute frequency and length of a note
fn process(note_wrapper: NoteComp, speed: f32) -> FranzResult<(f32, f32)> {
    let two: f32 = 2.0;
    let half: f32 = 0.5;

    // Match out note components. Compute frequency based on this.
    let note: ((BasePitch, Accidental, _), (BaseNoteLen, _)) =
        match note_wrapper {
            NoteComp::Note(n) => Ok(n),
            _ => Err(FranzError::FlattenError)
        }?;

    let (pitch, length) = note;
    let (base, acc, octave) = pitch;

    let idx = 12 * (octave + 1) + note_idx(&base, &acc);
    let diff: f32 = (idx - 69) as f32;
    let freq: f32 = 440.0 * two.powf(diff / 12.0);

    // Compute exact length of beat based on note value and dots
    let (beat, dots) = length;
    let mut growth_factor: f32 = 1.0;
    for i in 1..(dots + 1) {
        growth_factor += half.powf(i as f32);
    }

    // Truncated floating-point precision calculation for time
    let time: f32 =
        (1000.0 * speed * growth_factor * two.powf(length_idx(&beat) as f32))
            .round()
            / 1000.0;

    Ok((freq, time))
}

// Compile an AST to a series of notes, and write to file
pub fn compile_seq(
    name: &str, phrase: NoteComp, speed: f32
) -> FranzResult<()> {
    //Write results to file
    let mut file = File::create(format!("chuck-programs/{name}.ck"))
        .map_err(FranzError::IO)?;
    let (mut freq, mut time);

    let empty: Vec<((BasePitch, Accidental, i32), (BaseNoteLen, i32))> =
        Vec::new();

    let notes: Vec<((BasePitch, Accidental, i32), (BaseNoteLen, i32))> =
        match phrase {
            //Extract out note sequence, if present
            NoteComp::Phrase(v) => v,
            _ => empty
        };

    let _ = file.write_all(
        format!(
            "WvOut wav;\nwav.wavFilename(\"chuck-programs/{name}.wav\");\n"
        )
        .as_bytes()
    );

    let _ = file.write_all(b"SinOsc s => wav => dac;\n");

    for note in notes {
        //Process each line and write it

        (freq, time) = process(NoteComp::Note(note), speed)?;
        file.write_all(
            (format!(
                "0.5 => s.gain; {freq} => s.freq; {time} :: second => now;\n"
            ))
            .as_bytes()
        )
        .map_err(FranzError::IO)?;
    }
    let _ = file.write_all(b"wav.closeFile();");

    Ok(())
}
