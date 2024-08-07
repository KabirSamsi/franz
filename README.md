# Franz

Franz is a domain-specific programming & music description language. It aims to provide straightforward, text-based musical notation, and explores the relationship between rhythm motifs and note sequences.

Franz compiles to [ChucK](https://chuck.stanford.edu/). ChucK is a lower-level, strongly-timed programming language specifically targeted for music notation. It expresses notes at a frequency level. There are examples of `.ck` files in this project, the result of compilation. _Note that this compiler does not utilize the full functionality of ChucK, as it targets rather different aspects of musicality._

# Syntax

## Rhythm and Motifs 
Begin by describing any rhythm motifs as follows:

```
motif myMotif(<args>) = {
  <rhythm sequence>
};
```

`args` should contain names which can be either boolean or integer arguments. These can be used to specify details of the _rhythm sequence_.

A rhythm sequence can consist of beat values (`ts, sx, et, qt, hf, wh`) suffixed optionally with dots, which function the same way as dots in standard sheet music do. `hf` functions as a half note (two beats in common time) while `hf.` functions as a dotted-half note (three beats in common time). As so, a simple example to outline the rhythm of the opening to Beethoven's 5th symphony might be:

```
motif fifthSymphonyOpening() = {
  et; et; et; et; hf.;
};
```

(Note the four eighth beats, as opposed to three. The first one can be applied to a 'rest' pitch later to give the desired effect).

Rhythm sequences can be made more complicated using ternary operators and multiplication.

For example, here's a frequent rhythmic sequence with a few variations from the Star-Spangled Banner:

```
motif example(arg1) = {
  qt.; et; qt; qt; qt; (arg1 ? (qt; qt;) : (hf););
};
```

Or (to simplify our earlier example):

```
motif fifthSymphonyOpening() = {
  (4 * et); hf.;
};
```

Note that the beats should be included **after** the arithmetic expression.

Motifs are technically partial functions. They are intended to be applied to note sequences, inside the **control**, as we'll discuss momentarily.

## Defining a Control

A **control** has three components – a **use** block. a **phrases** list and a **return** block.

1) The **use** block outlines the parameters of the song (including time/key signature and tempo).
2) The **phrases** list will allow us to actually create components of the music.
3) The **return** block tells the compiler exactly what permutation of phrases to return.

These are wrapped around a `control` statement as such:

```
control {
  (* Instructions *)
};
```

### Use Blocks

Create `use` blocks as follows:

```
use {
  key = {<pitches in key signature>}
  meter = <count>/<beat type>;
  tempo = <tempo>;
};
```

Pitches (for key signatures) are octave-independent, and should be expressed as `f_shp` or `b_flt`. Tempo is currently limited to `Lento`, `Adagio`, `Andante`, `Allegro` and `Presto`. With this in mind, we could define the following sequence:

```
use {
  key = {b_flt, e_flt, a_flt};
  meter = 2/4;
  tempo = Allegro;
};
```

### Phrases

Phrases are possibly the most important component of a program.

The syntax for a `pitch` itself (without rhythm applied) is `<note><octave>_<optional accidental>` – for example, `c4_shp` or `b3_flt`. For natural notes and accidentals already specified via the key signature, just write `c4`. To re-naturalize a note, specify `c4_ntl`.

Pitch sequences are expressed as `{note1; note2; ...}` or for example, `{d4; e4; f4; g4; e4; c4; d4;}` `note`s themselves are written as a pairing `(c4, qt)`.

**Applying a motif to a pitch sequence** allows us to use earlier motifs pitch sequences to generate full note sequences. Here's the syntax for that:

```
beethoven5Opening = fifthSymphonyOpening({rst; g4; g4; g4; e4}) + fifthSymphonyOpening({rst; f4; f4; f4; d4});
```

Note that `e4` is not marked as flat. The key signature we described above handles that.

### Return Blocks

`Return` is actually just a phrase. Just chain together the desired phrases, with any necessary manipulations:

```
return {phrase1 + phrase2 + ...};
```

Or in this case:

```return {beethoven5Opening};```

# Footnotes

Part of my inspiration for this comes from [Calyx](https://docs.calyxir.org/intro.html), a research project and hardware-description IR I worked on!
