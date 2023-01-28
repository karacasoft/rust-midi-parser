pub mod midi_parse;
pub mod vlq;

#[cfg(test)]
mod tests {
    use super::midi_parse;
    use super::vlq;
    use crate::u32_from_vlq;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn create_midi_file_struct() {
        let _midi_file = midi_parse::MidiFile::from_file(String::from("RiverFlowsInYou.mid"));
        if _midi_file.is_err() {
            println!("{:?}", _midi_file.unwrap_err());
            panic!("Error occured while parsing MIDI");
        }
    }

    #[test]
    fn vlq_tests() {
        let bytes = [0x7f, 0, 0, 0];
        let my_vlq = vlq::Vlq::from(bytes);
        
        let decoded = u32::from(my_vlq);
        assert_eq!(decoded, 127);

        let bytes = [0x81, 0, 0, 0];
        let my_vlq = vlq::Vlq::from(bytes);
        
        let decoded = u32::from(my_vlq);
        assert_eq!(decoded, 128);

        let bytes = [0xff, 0xff, 0xff, 0x7f];
        let my_vlq = vlq::Vlq::from(bytes);
        
        let decoded = u32::from(my_vlq);
        assert_eq!(decoded, 0x0fffffff);

        // Testing the macro
        let decoded = u32_from_vlq!([0x7f, 0, 0, 0]);
        assert_eq!(decoded, 127);

        let decoded = u32_from_vlq!([0x81, 0, 0, 0]);
        assert_eq!(decoded, 128);

        let decoded = u32_from_vlq!([0xff, 0xff, 0xff, 0x7f]);
        assert_eq!(decoded, 0x0fffffff);

    }
}
