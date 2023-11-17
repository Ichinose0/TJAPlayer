use std::{fs::File, io::Error};

pub enum Course {
    Easy,
    Normal,
    Hard,
    Oni,
    Edit
}

impl From<u8> for Course {
    fn from(value: u8) -> Self {
        match value {
            0 => Course::Easy,
            1 => Course::Normal,
            2 => Course::Hard,
            3 => Course::Oni,
            4 => Course::Edit,
            _ => Course::Edit,
        }
    }
}

pub struct TJA {
    title: String,
    sub_title: String,
    level: u8,
    bpm: u16,
    wave: File,
    songvol: u8,
    sevol: u8,
    course: Course   
}

impl TJA {
    pub fn load(path: &str) -> Result<Self,Error> {
        let tja = match File::open(path) {
            Err(e) => {
                return Err(e);
            }
            Ok(t) => t
        };
        
        Ok(Self {
            title: "BLANK POST".to_owned(),
            sub_title: "美波".to_owned(),
            level: 10,
            bpm: 135,
            wave: File::open("blank-post.ogg").unwrap(),
            songvol: 100,
            sevol: 100,
            course: Course::Oni,
        })
    }
}