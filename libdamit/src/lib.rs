use elfloader::ElfBinary;
use loader::DamLoader;
use log::{debug, info};
use std::{error::Error, ffi::CString, iter::FromIterator};
use thiserror::Error;

mod loader;

#[derive(Debug, Error)]
pub enum BinaryError {
    #[error("Invalid Interp: {:?}", .0)]
    InvalidInterp(Vec<u8>),
    #[error("{}", .0)]
    Str(&'static str),
}

struct Segment {
    headers: Vec<elfloader::ProgramHeader64>,
    mem: (*mut u8, usize),
}

pub struct Binary {
    pub(crate) segments: Vec<Segment>,
    pub(crate) interp: Option<CString>,
}

impl Binary {
    pub fn new(data: Vec<u8>) -> Result<Self, BinaryError> {
        let mut bin = Binary {
            segments: Default::default(),
            interp: None,
        };
        let binary = ElfBinary::new(&data).expect("Got proper ELF file");
        for ph in binary.program_headers() {
            debug!("header: {:?}", ph);
            if matches!(ph.get_type(), Ok(xmas_elf::program::Type::Interp)) {
                let data = ph.get_data(&binary.file).map_err(BinaryError::Str)?;
                debug!("interp data: {:?}", data);
                match data {
                    xmas_elf::program::SegmentData::Undefined(dat) => {
                        // There is an unstable CString::from_vec_with_nul method:
                        // https://github.com/rust-lang/rust/issues/73179
                        // We should use it eventually.
                        let mut buf = Vec::from_iter(dat.into_iter().copied());
                        if matches!(buf.last(), Some(0)) {
                            buf.pop();
                        }
                        let interp_result = CString::new(buf)
                            // FIXME: this is not the original value, and
                            // it can be misleading.  We should use the unstable
                            // function above.
                            .map_err(|e| BinaryError::InvalidInterp(e.into_vec()))?;
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
