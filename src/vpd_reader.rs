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
use std::io::SeekFrom;

use crate::motion::*;

const VPD_HEADER: &str = "Vocaloid Pose Data file";

impl Motion {
    pub fn read_vpd(path: &Path) -> Motion {
        let raw_content = fs::read(path).unwrap();
        let content = WINDOWS_31J.decode(&raw_content, DecoderTrap::Ignore).unwrap();
        let content = content.trim();
        let split = content.split("\r\n\r\n");
        let mut model_name = String::new();
        let mut bone_keyframes: Vec<BoneKeyframe> = Vec::new();
        for (i, s) in split.enumerate() {
            if i == 0 {
                assert_eq!(*VPD_HEADER, *s);
            } else if i == 1 {
                model_name.push_str(s.split(".osm;").next().unwrap());
            } else {
                let mut name = String::new();
                let mut trans = [0f32; 3];
                let mut rot = [0f32; 4];
                for (j, s) in s.split_whitespace().enumerate() {
                    if j == 0 {
                        name.push_str(s.split("{").skip(1).next().unwrap());
                    } else if j == 1 {
                        let t: Vec<f32> = s.split(";").next().unwrap()
                            .split(",").map(|s| s.parse().unwrap())
                            .collect();
                        trans = [t[0], t[1], t[2]];
                    } else if j == 5 {
                        let r: Vec<f32> = s.split(";").next().unwrap()
                            .split(",").map(|s| s.parse().unwrap())
                            .collect();
                        rot = [r[0], r[1], r[2], r[3]];
                    }
                }
                bone_keyframes.push(BoneKeyframe {
                    name,
                    frame: 0,
                    trans,
                    rot,
                    txc: [0f32; 4],
                    tyc: [0f32; 4],
                    tzc: [0f32; 4],
                    rc:  [0f32; 4]
                });
            }
        }

        Motion {
            model_name,
            bone_keyframes,
            morph_keyframes:  Vec::new(),
            camera_keyframes: Vec::new(),
            light_keyframes:  Vec::new(),
            shadow_keyframes: Vec::new(),
        }
    }
}
