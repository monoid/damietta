use std::path::PathBuf;

use elfloader::ElfBinary;
use loader::DamLoader;

mod loader;

struct Segment {
    headers: Vec<elfloader::ProgramHeader64>,
    mem: (*mut u8, usize),
}

pub struct Binary {
    pub(crate) segments: Vec<Segment>,
}

impl Binary {
    pub fn new(data: Vec<u8>) -> Self {
        let mut bin = Binary {
            segments: Default::default(),
        };
        let binary = ElfBinary::new(&data).expect("Got proper ELF file");
        let mut loader = DamLoader { bin: &mut bin };
        binary.load(&mut loader).expect("Can't load the binary?");
        bin
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
