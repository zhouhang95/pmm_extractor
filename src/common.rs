use std::fs::File;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub fn read_float3(file: &mut File) -> [f32; 3] {
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

pub fn read_float4(file: &mut File) -> [f32; 4] {
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

pub fn read_items<T, F>(mut file: &mut File, f: F) -> Vec<T>
    where F: Fn(&mut File) -> T {
    let count = file.read_u32::<LittleEndian>().unwrap();
    read_fix_items(&mut file, count as usize, f)
}

pub fn read_fix_items<T, F>(file: &mut File, count: usize, f: F) -> Vec<T>
    where F: Fn(&mut File) -> T {
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
