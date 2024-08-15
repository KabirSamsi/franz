# Franz

Franz is a domain-specific programming & music description language. It aims to provide straightforward, text-based musical notation, and explores the relationship between rhythm motifs and note sequences.

Franz compiles to [ChucK](https://chuck.stanford.edu/), a low-level, strongly-timed programming language specifically targeted for music notation. It expresses notes at a frequency level. _Note that this compiler does not utilize the full functionality of ChucK, as it targets rather different aspects of musicality._

# Syntax

## Defining a Control

A **control**, the outer shell of the program, has three components – a **use** block. a **components** list and a **return** block.

1) The **use** block outlines the parameters of the song (including time/key signature and tempo).
2) The **components** list will allow us to actually create components of the music.
3) The **return** block tells the compiler exactly what permutation of phrases to return.

These are wrapped around a `control` statement as such:

```
control = {
  (* Instructions *)
};
```

### Use Blocks

Create `use` blocks as follows:

```
use {
  key = {<pitches in key signature>};
  meter = <count>/<beat type>;
  tempo = <tempo>;
};;
```

Pitches (for key signatures) are octave-independent, and should be expressed as `f_shp` or `b_flt`. Tempo is currently limited to `Lento`, `Adagio`, `Andante`, `Allegro` and `Presto`. With this in mind, we could define the following sequence:

```
use {
  key = {b_flt, e_flt, a_flt};
  meter = 2/4;
  tempo = Allegro;
};;
```

### Components

## Rhythm and Motifs 
Begin by describing any rhythm motifs. Note that all motifs, phrases and variables must be capitalized.

```
motif MyMotif[<Args>] = {
  <rhythm sequence>
};;
```

`args` should contain names which are boolean arguments. These can be used to specify details of the _rhythm sequence_.

A rhythm sequence can consist of beat values (`ts, sx, et, qt, hf, wh`) suffixed optionally with dots, which function the same way as dots in standard sheet music do. `hf` functions as a half note (two beats in common time) while `hf.` functions as a dotted-half note (three beats in common time). As so, a simple example to outline the rhythm of the opening to Beethoven's 5th symphony might be:

```
motif FifthSymphonyOpening[] = {
  et; et; et; hf.
};;
```

(Note the four eighth beats, as opposed to three. The first one can be applied to a 'rest' pitch later to give the desired effect).

Rhythm sequences can be made more complicated using ternary operators and multiplication.

For example, here's a frequent rhythmic sequence with a few variations from the Star-Spangled Banner:

```
motif Example[Arg1] = {
  qt.; et; {3 * qt}; {Arg1 ? (qt; qt) : hf}
};;
```

Or (to simplify our earlier example):

```
motif FifthSymphonyOpening[] = {
  (4 * et); hf.
};;
```

Note that the beats should be included **after** the arithmetic expression.

Motifs are technically partial functions. They are intended to be applied to note sequences, inside the **control**, as we'll discuss momentarily.

## Pitch Blocks

The syntax for a `pitch` itself (without rhythm applied) is `<note><octave>_<optional accidental>` – for example, `c4_shp` or `b3_flt`. For natural notes and accidentals already specified via the key signature, just write `c4`. To re-naturalize a note, specify `c4_ntl`.

Pitch sequences are expressed as `{note1; note2; ...}` or for example, `{d4; e4; f4; g4; e4; c4; d4;}` `note`s themselves are written as a pairing `(c4, qt)`.

**Applying a motif to a pitch sequence** allows us to use earlier motifs pitch sequences to generate full phrases. Here's the syntax for that:

```
phrase Beethoven5Opening = {
  FifthSymphonyOpening[{rst; {3 * g4}; e4}];
  FifthSymphonyOpening[{rst; {3 * f4}; d4}]
};;
```

Note that `e4` is not marked as flat. The key signature we described above handles that.

### Return Blocks

`return` can chain together the desired phrases, with any necessary manipulations, and return a phrase to end the whole sequence.

```
return {Phrase1; 2 * Phrase2; ...};;
```

Or in this case:

```return {Beethoven5Opening};;```
