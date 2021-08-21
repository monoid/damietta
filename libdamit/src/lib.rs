use elfloader::ElfBinary;
use loader::DamLoader;
use log::{debug, info};
use std::{error::Error, ffi::CString, iter::FromIterator};

mod loader;

struct Segment {
    headers: Vec<elfloader::ProgramHeader64>,
    mem: (*mut u8, usize),
}

pub struct Binary {
    pub(crate) segments: Vec<Segment>,
    pub(crate) interp: Option<CString>,
}

impl Binary {
    pub fn new(data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let mut bin = Binary {
            segments: Default::default(),
            interp: None,
        };
        let binary = ElfBinary::new(&data).expect("Got proper ELF file");
        for ph in binary.program_headers() {
            debug!("header: {:?}", ph);
            if matches!(ph.get_type(), Ok(xmas_elf::program::Type::Interp)) {
                let data = ph.get_data(&binary.file)?;
                debug!("interp data: {:?}", data);
                match data {
                    xmas_elf::program::SegmentData::Undefined(dat) => {
                        let mut buf = Vec::from_iter(dat.into_iter().copied());
                        if matches!(buf.last(), Some(0)) {
                            buf.pop();
                        }
                        let interp_result = CString::new(buf)?;
                        info!("interp: {:?}", interp_result);
                        bin.interp = Some(interp_result);
                    }
                    _ => {
                        info!("Unexpected Interp data: {:?}, expecting Undefined", data);
                    }
                }
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
