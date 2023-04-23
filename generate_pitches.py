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
print(base_str)
# print(set_of_lengths)