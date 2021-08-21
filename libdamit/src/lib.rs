use elfloader::ElfBinary;
use loader::DamLoader;
use log::{debug, info};
use std::error::Error;

mod loader;

struct Segment {
    headers: Vec<elfloader::ProgramHeader64>,
    mem: (*mut u8, usize),
}

pub struct Binary {
    pub(crate) segments: Vec<Segment>,
}

impl Binary {
    pub fn new(data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let mut bin = Binary {
            segments: Default::default(),
        };
        let binary = ElfBinary::new(&data).expect("Got proper ELF file");
        let mut interp = None;
        for ph in binary.program_headers() {
            debug!("header: {:?}", ph);
            if matches!(ph.get_type(), Ok(xmas_elf::program::Type::Interp)) {
                interp = Some(ph.get_data(&binary.file)?);
                info!("interp: {:?}", interp.unwrap());
            }
        }

        let mut loader = DamLoader { bin: &mut bin };
        binary.load(&mut loader).expect("Can't load the binary?");
        Ok(bin)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
