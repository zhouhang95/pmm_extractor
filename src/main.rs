#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod common;
mod pmm;
mod motion;
mod vmd_reader;
mod vmd_writer;
mod vpd_reader;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_31J;

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::SeekFrom;

use motion::*;
use vmd_reader::*;
use vmd_writer::*;
use vpd_reader::*;

use common::read_float3;
use common::read_float4;
use crate::pmm::read_pmm;


fn main() {
    let path = Path::new("crack.pmm");
    let motions = read_pmm(&path);
    for (i, m) in motions.iter().enumerate() {
        m.write_vmd(&format!("{:0>2}.vmd", i));
    }
}
