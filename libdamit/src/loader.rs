use crate::Binary;
use elfloader::TypeRela64;
use log::info;

pub(crate) struct DamLoader<'a> {
    pub(crate) bin: &'a mut Binary,
}

impl<'a> elfloader::ElfLoader for DamLoader<'a> {
    fn allocate(
        &mut self,
        load_headers: elfloader::LoadableHeaders,
    ) -> Result<(), elfloader::ElfLoaderErr> {
        for header in load_headers {
            info!(
                "allocate base = {:#x} size = {:#x} flags = {}",
                header.virtual_addr(),
                header.mem_size(),
                header.flags()
            );
        }
        Ok(())
    }

    fn load(
        &mut self,
        flags: elfloader::Flags,
        base: elfloader::VAddr,
        region: &[u8],
    ) -> Result<(), elfloader::ElfLoaderErr> {
        let start = base;
        let end = base + region.len() as u64;
        info!("load region into = {:#x} -- {:#x}", start, end);
        Ok(())
    }

    fn relocate(
        &mut self,
        entry: &elfloader::Rela<elfloader::P64>,
    ) -> Result<(), elfloader::ElfLoaderErr> {
        let typ = TypeRela64::from(entry.get_type());
        let addr: *mut u64 = (entry.get_offset()) as *mut u64;

        match typ {
            TypeRela64::R_RELATIVE => {
                // This is a relative relocation, add the offset (where we put our
                // binary in the vspace) to the addend and we're done.
                info!("R_RELATIVE *{:p} = {:#x}", addr, entry.get_addend());
                Ok(())
            }
            _ => {
                info!("Something: {:?}", typ);
                Ok((/* not implemented */))
            }
        }
    }
}
