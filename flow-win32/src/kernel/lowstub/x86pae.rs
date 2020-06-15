use crate::error::{Error, Result};
use crate::kernel::StartBlock;

use byteorder::{ByteOrder, LittleEndian};

use flow_core::architecture::{self, Architecture};
use flow_core::iter::PageChunks;
use flow_core::types::Address;

fn check_page(addr: Address, mem: &[u8]) -> bool {
    for (i, chunk) in mem.to_vec().chunks_exact(8).enumerate() {
        if i < 4 && LittleEndian::read_u64(chunk) != addr.as_u64() + ((i as u64 * 8) << 9) + 0x1001
        {
            return false;
        } else if i >= 4 && LittleEndian::read_u64(chunk) != 0 {
            return false;
        }
    }
    true
}

pub fn find(mem: &[u8]) -> Result<StartBlock> {
    mem.page_chunks(Address::from(0), architecture::x86_pae::page_size())
        .find(|(a, c)| check_page(a.clone(), c))
        .ok_or_else(|| Error::new("unable to find x64_pae dtb in lowstub < 16M"))
        .and_then(|(a, _)| {
            Ok(StartBlock {
                arch: Architecture::X86Pae,
                va: Address::from(0),
                dtb: a,
            })
        })
}
