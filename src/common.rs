use std::fs::File;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

pub fn read_float3<T>(file: &mut T) -> [f32; 3]
    where T: Read {
    let x = file.read_f32::<LittleEndian>().unwrap();
    let y = file.read_f32::<LittleEndian>().unwrap();
    let z = file.read_f32::<LittleEndian>().unwrap();
    [x, y, z]
}
pub fn write_float3(file: &mut File, [x, y, z]: [f32; 3]) {
    file.write_f32::<LittleEndian>(x).unwrap();
    file.write_f32::<LittleEndian>(y).unwrap();
    file.write_f32::<LittleEndian>(z).unwrap();
}

pub fn read_float4<T>(file: &mut T) -> [f32; 4]
    where T: Read {
    let x = file.read_f32::<LittleEndian>().unwrap();
    let y = file.read_f32::<LittleEndian>().unwrap();
    let z = file.read_f32::<LittleEndian>().unwrap();
    let w = file.read_f32::<LittleEndian>().unwrap();
    [x, y, z, w]
}
pub fn write_float4(file: &mut File, [x, y, z, w]: [f32; 4]) {
    file.write_f32::<LittleEndian>(x).unwrap();
    file.write_f32::<LittleEndian>(y).unwrap();
    file.write_f32::<LittleEndian>(z).unwrap();
    file.write_f32::<LittleEndian>(w).unwrap();
}

pub fn read_items<R, T, F>(file: &mut R, f: F) -> Vec<T>
    where R: Read, F: Fn(&mut R) -> T {
    let count = file.read_u32::<LittleEndian>().unwrap() as usize;
    let mut items: Vec<T> = Vec::with_capacity(count);
    for _ in 0..count {
        items.push(f(file));
    }
    items
}

pub fn read_fix_items<R, T, F>(file: &mut R, count: usize, f: F) -> Vec<T>
    where R: Read, F: Fn(&mut R) -> T {
    let mut items: Vec<T> = Vec::with_capacity(count);
    for _ in 0..count {
        items.push(f(file));
    }
    items
}
pub fn write_items<T, F>(mut file: &mut File, content: &Vec<T>, f: F)
    where F: Fn(&mut File, &T) -> () {
    let count = content.len();
    file.write_u32::<LittleEndian>(count as u32).unwrap();
    for i in 0..count {
        f(&mut file, &content[i]);
    }
}
