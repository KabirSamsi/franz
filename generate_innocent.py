with open("test.txt", "w") as f:
    f.write("SinOsc s => dac;")


notes = ("C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B")

def calculate_frequency(note):
    if note == 0:
        return 16.35
    if note > 0 and note < 12:
        return 1.0595 * calculate_frequency(note - 1)
    if note >= 12:
        return 2 * calculate_frequency(note - 12)

print(calculate_frequency(notes.index("F#")))