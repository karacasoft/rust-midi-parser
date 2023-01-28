# rust-midi-parser

This is a practice project to learn Rust. It parses MIDI files. That's all.

## Usage

```Rust

let midi_file = midi_parse::MidiFile::from_file(String::from("SomeMIDIFile.mid"));

println!("{:?}", midi_file.tracks);

```

I'm too lazy to add a documentation, please look at the source code for the fields and stuff.