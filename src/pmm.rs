#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_31J;

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{SeekFrom, Cursor};
use crate::vmd_reader::{read_string};
use crate::common::{read_items, read_fix_items, read_float3, read_float4};
use crate::motion::{BoneKeyframe, MorphKeyframe, Motion};
use std::collections::HashMap;

const PMM_HEADER: &str = "Polygon Movie maker 0002";

fn read_v_string<T>(file: &mut T) -> String
    where T: Read {
    let len = file.read_u8().unwrap() as usize;
    let mut string_raw = vec![0u8; len];
    file.read(&mut string_raw).unwrap();
    WINDOWS_31J.decode(&string_raw, DecoderTrap::Ignore).unwrap()
}

pub fn read_header<T>(mut file: &mut T)
        where T: Read {
    let header_string = read_string(&mut file, 30);
    assert!(header_string.starts_with(PMM_HEADER));
    let view_width = file.read_u32::<LittleEndian>().unwrap();
    let view_height = file.read_u32::<LittleEndian>().unwrap();
    let frame_width = file.read_u32::<LittleEndian>().unwrap();
    let edit_view_angle = file.read_f32::<LittleEndian>().unwrap();

    let camera_light_accessory_edited = file.read_u8().unwrap() == 1;
    let camera_panel_opened = file.read_u8().unwrap() == 1 ;
    let light_panel_opened = file.read_u8().unwrap() == 1 ;
    let accessory_panel_opened = file.read_u8().unwrap() == 1 ;
    let bone_panel_opened = file.read_u8().unwrap() == 1 ;
    let morph_panel_opened = file.read_u8().unwrap() == 1 ;
    let self_shadow_panel = file.read_u8().unwrap() == 1 ;
    let selected_model_index = file.read_u8().unwrap();
}
pub fn read_model<T>(mut file: &mut T) -> Motion
    where T: Read {
    let number = file.read_u8().unwrap();
    let name = read_v_string(&mut file);
    let name_en = read_v_string(&mut file);
    let path = read_string(&mut file, 256);
    let keyframe_editor_top_level_rows = file.read_u8().unwrap();
    let bone_names = read_items(&mut file, read_v_string);
    let morph_names = read_items(&mut file, read_v_string);
    let ik_indexes = read_items(&mut file, |file| {
        file.read_u32::<LittleEndian>().unwrap()
    });
    let op_indexes = read_items(&mut file, |file| {
        file.read_u32::<LittleEndian>().unwrap()
    });
    let draw_order = file.read_u8().unwrap();
    let edit_is_display = file.read_u8().unwrap() == 1;
    let edit_selected_bone = file.read_u32::<LittleEndian>().unwrap();
    let skin_panel = read_fix_items(&mut file, 4, |file| {
        file.read_u32::<LittleEndian>().unwrap()
    });

    let frame_opened_count = file.read_u8().unwrap() as usize;
    let frame_opened = read_fix_items(&mut file, frame_opened_count, |file| {
        file.read_u8().unwrap()
    });
    let v_scroll = file.read_u32::<LittleEndian>().unwrap();
    let last_frame = file.read_u32::<LittleEndian>().unwrap();

    let mut bone_key_frames: HashMap<u32, BoneKeyframe> = HashMap::new();
    for _ in 0..bone_names.len() {
        read_bone_frame(&mut file, &mut bone_key_frames, &bone_names);
    }
    let remaining_bone_frame = file.read_u32::<LittleEndian>().unwrap();
    for _ in 0..remaining_bone_frame {
        read_bone_frame(&mut file, &mut bone_key_frames, &bone_names);
    }

    let mut morph_key_frames: HashMap<u32, MorphKeyframe> = HashMap::new();
    for _ in 0..morph_names.len() {
        read_morph_frame(&mut file, &mut morph_key_frames, &morph_names);
    }
    let remaining_morph_frame = file.read_u32::<LittleEndian>().unwrap();
    for _ in 0..remaining_morph_frame {
        read_morph_frame(&mut file, &mut morph_key_frames, &morph_names);
    }

    let op_init_frame = read_op_frame(&mut file, ik_indexes.len(), op_indexes.len(), true);
    let op_key_frames = read_items(&mut file, |mut file| {
        read_op_frame(&mut file, ik_indexes.len(), op_indexes.len(), false);
    });
    let bone_current_data = read_fix_items(&mut file, bone_names.len(), |mut file| {
        read_bone_current_data(&mut file)
    });
    let morph_current_datas = read_fix_items(&mut file, morph_names.len(), |file| {
        file.read_f32::<LittleEndian>().unwrap()
    });
    let is_current_ik_enabled_data = read_fix_items(&mut file, ik_indexes.len(), |file| {
        file.read_u8().unwrap() == 1
    });
    let op_current_data = read_fix_items(&mut file, op_indexes.len(), |mut file| {
        read_op_current_data(&mut file)
    });
    let blend_added = file.read_u8().unwrap() == 1;
    let edge_width = file.read_f32::<LittleEndian>().unwrap();
    let self_shadow_enabled = file.read_u8().unwrap() == 1;
    let calc_order = file.read_u8().unwrap();



    let mut bone_frame_list = vec![];
    for i in 0..bone_names.len() {
        let name = bone_names[i].clone();
        let mut index = i;
        loop {
            let kf =  &bone_key_frames[&(index as u32)];
            let next: usize = kf.name.parse().unwrap();
            bone_frame_list.push(BoneKeyframe {
                name: name.clone(),
                frame: kf.frame,
                trans: kf.trans,
                rot: kf.rot,
                txc: kf.txc,
                tyc: kf.tyc,
                tzc: kf.tzc,
                rc: kf.rc,
            });
            if next != 0 {
                index = next;
            } else {
                break;
            }
        }
    }


    let mut morph_frame_list = vec![];
    for i in 0..morph_names.len() {
        let name = morph_names[i].clone();
        let mut index = i;
        loop {
            let kf =  &morph_key_frames[&(index as u32)];
            let next: usize = kf.name.parse().unwrap();
            morph_frame_list.push(MorphKeyframe {
                name: name.clone(),
                frame: kf.frame,
                weight: kf.weight,
            });
            if next != 0 {
                index = next;
            } else {
                break;
            }
        }
    }

    Motion {
        model_name: name,
        bone_keyframes: bone_frame_list,
        morph_keyframes: morph_frame_list,
        camera_keyframes: vec![],
        light_keyframes: vec![],
        shadow_keyframes: vec![]
    }
}

pub fn read_bone_frame<T>(mut file: &mut T,
                       keyframes: &mut HashMap<u32, BoneKeyframe>, names: &Vec<String>)
    where T: Read {
    let data_index = if keyframes.len() < names.len() {
        keyframes.len() as u32
    } else {
        file.read_u32::<LittleEndian>().unwrap()
    };
    let frame = file.read_u32::<LittleEndian>().unwrap();
    let pre_index = file.read_u32::<LittleEndian>().unwrap() as usize;
    let next_index = file.read_u32::<LittleEndian>().unwrap() as usize;
    let txc = read_fix_items(&mut file, 4, |file| {
        file.read_u8().unwrap() as f32 / 127f32
    });
    let tyc = read_fix_items(&mut file, 4, |file| {
        file.read_u8().unwrap() as f32 / 127f32
    });
    let tzc = read_fix_items(&mut file, 4, |file| {
        file.read_u8().unwrap() as f32 / 127f32
    });
    let rc = read_fix_items(&mut file, 4, |file| {
        file.read_u8().unwrap() as f32 / 127f32
    });
    let trans = read_float3(&mut file);
    let rot = read_float4(&mut file);
    let selected = file.read_u8().unwrap() == 1;
    let physics_disabled = file.read_u8().unwrap() == 1;


    keyframes.insert(data_index,  BoneKeyframe {
        name: next_index.to_string(),
        frame,
        trans,
        rot,
        txc: [txc[0], txc[1], txc[2], txc[3]],
        tyc: [tyc[0], tyc[1], tyc[2], tyc[3]],
        tzc: [tzc[0], tzc[1], tzc[2], tzc[3]],
        rc:  [rc[0], rc[1], rc[2], rc[3]],
    });
}

pub fn read_morph_frame<T>(file: &mut T,
                         keyframes: &mut HashMap<u32, MorphKeyframe>, names: &Vec<String>)
    where T: Read {
    let data_index = if keyframes.len() < names.len() {
        keyframes.len() as u32
    } else {
        file.read_u32::<LittleEndian>().unwrap()
    };
    let frame = file.read_u32::<LittleEndian>().unwrap();

    let pre_index = file.read_u32::<LittleEndian>().unwrap() as usize;
    let next_index = file.read_u32::<LittleEndian>().unwrap() as usize;

    let weight = file.read_f32::<LittleEndian>().unwrap();
    let selected = file.read_u8().unwrap() == 1;

    keyframes.insert(data_index,  MorphKeyframe {
        name: next_index.to_string(),
        frame,
        weight,
    });
}
pub fn read_op_frame<T>(mut file: &mut T, ik_count: usize, op_count: usize, inited: bool)
    where T: Read {
    let data_index = if inited { -1 } else { file.read_i32::<LittleEndian>().unwrap() };
    let frame = file.read_u32::<LittleEndian>().unwrap();
    let pre_index = file.read_u32::<LittleEndian>().unwrap();
    let next_index = file.read_u32::<LittleEndian>().unwrap();
    let displayed = file.read_u8().unwrap() == 1;
    let ik_enabled = read_fix_items(&mut file, ik_count, |file| {
        file.read_u8().unwrap() == 1
    });
    let op_data = read_fix_items(&mut file, op_count, |file| {
        (file.read_u32::<LittleEndian>().unwrap(),
         file.read_u32::<LittleEndian>().unwrap())
    });
    let selected = file.read_u8().unwrap() == 1;
}

fn read_op_current_data<T>(file: &mut T)
    where T: Read {
    let key_frame_begin = file.read_u32::<LittleEndian>().unwrap();
    let key_frame_end = file.read_u32::<LittleEndian>().unwrap();
    let model_index = file.read_u32::<LittleEndian>().unwrap();
    let parent_bone_index = file.read_u32::<LittleEndian>().unwrap();
}

fn read_bone_current_data<T>(mut file: &mut T)
    where T: Read {
    let trans = read_float3(&mut file);
    let rot = read_float4(&mut file);
    let edit_un_commited = file.read_u8().unwrap() == 1;
    let physics_disabled = file.read_u8().unwrap() == 1;
    let row_selected = file.read_u8().unwrap() == 1;
}

pub fn read_pmm(path: &Path) -> Vec<Motion> {
    let content = fs::read(path).unwrap();
    let mut file = Cursor::new(content);
    read_header(&mut file);
    let count = file.read_u8().unwrap() as usize;
    read_fix_items(&mut file, count, read_model)
}
