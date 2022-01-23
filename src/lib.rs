use std::fmt;
use std::fs::OpenOptions;
use std::io::{BufReader, Read, Write};

mod defines;

pub struct Iso {
    udf: bool,
    patched: bool,
    path: String,
    data: Vec<u8>,
}

// Public Functions
impl Iso {
    pub fn new(path: &str) -> Iso {
        let iso_f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(false)
            .open(path)
            .expect("Could not open file");
        let mut reader = BufReader::new(iso_f);
        let mut data: Vec<u8> = vec![];
        reader
            .read_to_end(&mut data)
            .expect("Could not read file into memory.");
        let mut iso = Iso {
            udf: false,
            patched: false,
            path: path.to_string(),
            data,
        };
        iso.check_iso();
        iso
    }
    pub fn check_iso(&mut self) {
        self.check_udf();
        self.check_patched();
    }
    pub fn check_udf(&mut self) -> bool {
        self.udf = false;
        for i in 1..64 {
            let offset: usize = ((LBA_SIZE * i) + 32768 + 1)
                .try_into()
                .expect("Quick math!");
            if self.data[offset..offset + 3] == b"NSR".to_owned() {
                self.udf = true;
                break;
            }
        }
        return self.udf;
    }

    pub fn check_patched(&mut self) -> bool {
        let offset = ((LBA_SIZE * 14) + 25) as usize;
        self.patched = self.data[offset..offset + 4] == b"+NSR".to_owned();
        self.patched
    }

    pub fn patch(&mut self) {
        if !self.udf {
            panic!("No UDF descriptor found. Is this really an ISO?")
        }
        if self.patched {
            panic!("Already patched! Did you want to unpatch?");
        }
        self.copy_lba(34, 14);
        self.copy_lba(50, 15);
        self.patch_lba(34);
        self.patch_lba(50);
        self.write_dvd_data();
        self.check_iso();
    }

    pub fn unpatch(&mut self) {
        if !self.udf {
            panic!("No UDF descriptor found. Is this really an ISO?")
        }
        if !self.patched {
            panic!("File isn't patched! Did you want to patch?");
        }
        self.copy_lba(14, 34);
        self.copy_lba(15, 50);
        let zeros = vec![0u8; LBA_SIZE as usize];
        self.write_lba(&zeros, 14);
        self.write_lba(&zeros, 15);
        for i in 0..12 {
            self.write_lba(&zeros, 128 + i);
        }
        self.check_iso();
    }
    pub fn write(&self, filename: &str) {
        let mut iso_f = OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .open(filename)
            .expect("Could not open file for writing");
        iso_f.write_all(&self.data).expect("Could not write data to file.");
    }
}

// Private Functions
impl Iso {
    fn patch_lba(&mut self, dst_lba: u64) {
        let dst_s = (dst_lba * LBA_SIZE) as usize;
        let dst_e = dst_s + LBA_SIZE as usize;
        let mut lba = self.data[dst_s..dst_e].to_vec();

        lba[188] = 128;
        lba[189] = 0;

        let desc_crc_len: u16 = (lba[10] as u16) |  ((lba[11] as u16) << 8);
        let desc: Vec<u8> = lba[16..2048].to_vec();
        let desc_crc = Self::crc(&desc, desc_crc_len as usize);
        let desc_crc_bytes = desc_crc.to_le_bytes();
        lba[8] = desc_crc_bytes[0];
        lba[9] = desc_crc_bytes[1];

        let mut checksum = 0u8;
        for i in 0..16 {
            checksum = checksum.wrapping_add(lba[i]);
        }
        checksum = checksum.wrapping_sub(lba[4]);
        lba[4] = checksum;
        self.write_lba(&lba, dst_lba);
    }

    fn write_lba(&mut self, lba: &Vec<u8>, dst_lba: u64) {
        let dst_s= (dst_lba * LBA_SIZE) as usize;
        let dst_e = dst_s + (LBA_SIZE as usize);
        self.data[dst_s..dst_e].copy_from_slice(&lba);
    }

    fn copy_lba(&mut self, slba: u64, dlba: u64) {
        let src_start  = (LBA_SIZE * slba) as usize;
        let src_end = src_start + (LBA_SIZE as usize);
        let lba = self.data[src_start..src_end].to_vec();
        let dest_start = (LBA_SIZE * dlba) as usize;
        let dest_end = dest_start + (LBA_SIZE as usize);
        self.data[dest_start..dest_end].copy_from_slice(&lba[..])
    }

    fn write_dvd_data(&mut self) {
        let dst_s = (128 * LBA_SIZE) as usize;
        let dst_e = dst_s + 12 * (LBA_SIZE as usize);
        self.data[dst_s..dst_e].copy_from_slice(&defines::DVD_DATA);
    }
}

// Associated functions
impl Iso {
    fn crc(block: &[u8], desc_crc_len: usize) -> u16 {
        let mut crc = 0u16;
        for i in 0..desc_crc_len {
            let crc_bytes = crc.to_le_bytes();
            let crc_h = crc_bytes[0];
            let crc_l = crc_bytes[1];
            crc = (crc_h as u16) << 8;
            let j: usize = (crc_l ^ block[i]).into();
            crc ^= defines::CRC_LOOKUP[j];
        }
        crc
    }
}

impl fmt::Display for Iso {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Path:\t{}\nUDF:\t{}\nPatch:\t{}",
            self.path, self.udf, self.patched
        )
    }
}

const LBA_SIZE: u64 = 2048;