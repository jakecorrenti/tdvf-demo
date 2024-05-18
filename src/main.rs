/// https://github.com/tianocore/edk2/blob/284dbac43da752ee34825c8b3f6f9e8281cb5a19/OvmfPkg/ResetVector/Ia16/ResetVectorVtf0.asm#L35
/// https://github.com/tianocore/edk2/blob/284dbac43da752ee34825c8b3f6f9e8281cb5a19/OvmfPkg/ResetVector/X64/IntelTdxMetadata.asm#L4
use std::io::{Read, Seek, SeekFrom};
use uuid::{Error, Uuid};

const EXPECTED_TABLE_FOOTER_GUID: &str = "96b582de-1fb2-45f7-baea-a366c55a082d";
const EXPECTED_METADATA_OFFSET_GUID: &str = "e47a6535-984a-4798-865e-4685a7bf8ec2";

// TODO: better document this
fn locate_table_footer_guid(fd: &mut std::fs::File) -> Result<Uuid, Error> {
    // move the position in the file to 0xFFFFFFFF - 0x30, or 0xFFFFFFCF which is right before
    // where the table footer guid should be located
    fd.seek(SeekFrom::End(-0x30))
        .expect("Unable to seek to the offset in the file");

    let mut table_footer_guid: [u8; 16] = [0; 16];
    fd.read_exact(&mut table_footer_guid)
        .expect("Unable to read the exact amount of bytes required to fill the buffer");

    Uuid::from_slice_le(table_footer_guid.as_slice())
}

// TODO: document
fn locate_table_size(fd: &mut std::fs::File) -> Result<usize, Error> {
    fd.seek(SeekFrom::End(-0x32))
        .expect("Unable to seek to the offset in the file.");

    let mut table_size: [u8; 2] = [0; 2];
    fd.read_exact(&mut table_size)
        .expect("Unable to read the exact amount of bytes required to fill the buffer");

    Ok(u16::from_le_bytes(table_size) as usize)
}

fn main() {
    let mut firmware_path = std::fs::File::open("/usr/share/edk2/ovmf/OVMF.inteltdx.fd").unwrap();
    let located = locate_table_footer_guid(&mut firmware_path).unwrap();
    let expected =
        Uuid::parse_str(EXPECTED_TABLE_FOOTER_GUID).expect("Unable to parse string into Uuid");

    // we found the table footer guid
    if located == expected {
        // find the table size
        let table_size =
            locate_table_size(&mut firmware_path).expect("Unable to locate TDVF table size");

        // TODO: read the file contents into the table
        let mut table: Vec<u8> = vec![0; table_size];
        println!("{}", table_size);
    }
}
