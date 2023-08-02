import music21 as m21

# We want to generate a list of chords which themselves contain a list of pitches
# and we want to output them as a rust array.
base_str = "let mut PITCHES: Vec<Vec<f32>> = Vec::new();"
len_to_be_replaced = "UNKNOWN_NOW"

# We want all sort of chords, from normal chords like C major to chords like C major 7th
# we want to iterate over all the supported chords and generate a list of them
chords = []
for note_range in range(12):
    for chord_type in m21.harmony.CHORD_TYPES:
        root = m21.pitch.Pitch(note_range)
        chord = m21.harmony.ChordSymbol(root=root, kind=chord_type)
        # we want to store the chord as a list of pitches which are frequencies
        chord_freqs = [p.frequency for p in chord.pitches]
        chord_name = chord.pitchedCommonName
        chords.append((chord_name, chord_freqs))


# print(chords)
base_str = base_str.replace(len_to_be_replaced, str(len(chords)))

chord_types = set()
for chord_type in m21.harmony.CHORD_TYPES:
    chord_types.add(chord_type)

# print(chord_types)
# Get the theory behind the chord types
# Importing the music21 chord module
import music21.chord as chord
import re

rust_struct_syntax_chord_types = []
rust_struct_chord_types = []

for chord_type in chord_types:
    # we want to convert the chord type to a rust struct syntax
    # for example: "major" -> "Major"
    # for example: "minor" -> "Minor"
    # for example: "something_something" -> "SomethingSomething"
    # for example: "something_something_else" -> "SomethingSomethingElse"
    # ...
    rust_str = ""
    # we want to split with space and - and _ and then capitalize each word
    words = re.split(" |-|_", chord_type)
    for word in words:
        rust_str += word.capitalize()
    rust_struct_chord_types.append(rust_str)
    rust_struct_str = "pub struct " + rust_str + " {" + "}"
    rust_struct_syntax_chord_types.append(rust_struct_str)

print(rust_struct_syntax_chord_types)
rust_struct_syntax_chord_types = sorted(rust_struct_syntax_chord_types)
for rssct in rust_struct_syntax_chord_types:
    print(rssct)
    print()
    
print("pub enum ChordType {")
for rsct in rust_struct_chord_types:
    print(f"    {rsct}({rsct}),")
print("}")



# Now I want to generate the impl part that creates a new chord
# It must take three arguments, the root and the octave, and the chord type
# We must match the chord type and then create the chord
# We must return the chord
print("impl Chord {")
print("    pub fn new(root: Note, octave: u8, chord_type: ChordType) -> Chord {")
print("        match chord_type {")
for rsct in rust_struct_chord_types:
    print(f"            ChordType::{rsct} => {rsct}::new(root, octave),")
print("        }")
print("    }")

print("pub trait ChordBuilder {")
print("    fn new(root: Note, octave: u8) -> Chord;")
print("}")

for rsct in rust_struct_chord_types:
    print(f"pub impl ChordBuilder for {rsct}" + "{")
    print(f"    fn new(root: Note, octave: u8) -> Chord" + "{")
    if rsct == 'Augmented':
        print("        let mut chord = Chord::new(root, octave, ChordType::Major);")
        print("        chord.add_interval(Interval::AugmentedFifth);")
        print("        chord")
    elif rsct == 'Augmented11th':
        print("        let mut chord = Chord::new(root, octave, ChordType::Major);")
        print("        chord.add_interval(Interval::AugmentedFifth);")
        print("        chord.add_interval(Interval::Perfect11th);")
        print("        chord")


for rsct in rust_struct_chord_types:
    print(rsct)


set_of_lengths = set()
for chord_name, chord_freqs in chords:
    # if len(chord_freqs) > 3:
    #     print("Warning: chord has more than 3 notes")
    set_of_lengths.add(len(chord_freqs))
    # while len(chord_freqs) < 7:
    #     chord_freqs.append(0.0)
    base_str += f"""
        // {chord_name}
        PITCHES.push(vec!["""
    for i, freq in enumerate(chord_freqs):
        base_str += f"{freq}"
        if i != len(chord_freqs) - 1:
            base_str += ", "
    base_str += "]);"

# base_str += "];"
# print(base_str)
# print(set_of_lengths)
