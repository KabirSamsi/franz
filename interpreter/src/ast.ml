type varname = string

type beattype =
  Whole | Half | Quarter | Eighth | Sixteenth | ThirtySecond

type note = 
  C | Cb | Cs | D | Db | Ds | E | Eb | Es | F | Fb | Fs | G | Gb | Gs | A | Ab | As | B | Bb | Bs

type resttype = 
  Whole | Half | Quarter | Eighth | Sixteenth | ThirtySecond

type timesig = int * int
type keysig = Empty | Acc of note | keysig * keysig 

type expr = 
| EEmpty
| EBeat of beattype
| ENote of note * int
| ERest of resttype
| EPhrase of expr * expr

(* Rhythm Sequence *)
| ERhythSeq of expr * expr 

(* Note Sequence *)
| ENoteSeq of expr * expr

(* Apply rhythm sequence to note sequence *)
| ERhythApp of expr * expr