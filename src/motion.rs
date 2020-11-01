pub struct Motion {
    pub model_name:       String,
    pub bone_keyframes:   Vec<BoneKeyframe>,
    pub morph_keyframes:  Vec<MorphKeyframe>,
    pub camera_keyframes: Vec<CameraKeyframe>,
    pub light_keyframes:  Vec<LightKeyframe>,
    pub shadow_keyframes: Vec<ShadowKeyframe>,
}

pub struct BoneKeyframe {
    pub name:  String,
    pub frame: u32,
    pub trans: [f32; 3],
    pub rot:   [f32; 4],
    pub txc:   [f32; 4],
    pub tyc:   [f32; 4],
    pub tzc:   [f32; 4],
    pub rc:    [f32; 4],
}

pub struct MorphKeyframe {
    pub name:   String,
    pub frame:  u32,
    pub weight: f32,
}

pub struct CameraKeyframe {
    pub frame: u32,
    pub dist:  f32,
    pub trans: [f32; 3],
    pub rot:   [f32; 3],
    pub txc:   [f32; 4],
    pub tyc:   [f32; 4],
    pub tzc:   [f32; 4],
    pub rc :   [f32; 4],
    pub dc :   [f32; 4],
    pub vc :   [f32; 4],
    pub fov:   u32,
    pub perspective: bool,
}

pub struct LightKeyframe {
    pub frame:     u32,
    pub color:     [f32; 3],
    pub direction: [f32; 3],
}

pub struct ShadowKeyframe {
    pub frame: u32,
    pub mode:  u8,
    pub dist:  f32,
}

impl Motion {
    pub fn new() -> Motion {
        Motion {
            model_name:       String::new(),
            bone_keyframes:   Vec::new(),
            morph_keyframes:  Vec::new(),
            camera_keyframes: Vec::new(),
            light_keyframes:  Vec::new(),
            shadow_keyframes: Vec::new(),
        }
    }
}