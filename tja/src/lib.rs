use std::{fs::File, io::{Error, Read, BufReader, BufRead}};

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

pub struct Score {
    score_string: String,
    measure: u32
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
        let mut title = String::new();
        let mut sub_title = String::new();
        let mut wave = String::new();

        let mut f = match File::open(path) {
            Err(e) => {
                return Err(e);
            }
            Ok(t) => t
        };

        let mut tja = vec![];

        let reader = BufReader::new(f);
        for line in reader.lines() {
            let line = line.unwrap();
            let bytes = line.as_bytes();
            if bytes.len() != 0 {
                if bytes[0] as char != '/' {
                    tja.push(line);
                }
            }
        }

        let mut start = 0;
        let mut end = 0;

        for (i,items) in tja.iter().enumerate() {
            if items.as_bytes()[0] as char == 'T' {
                if &items[..5] == "TITLE" {
                    title = items[6..].to_owned();
                }
            }
            if items.as_bytes()[0] as char == 'S' {
                if &items[..8] == "SUBTITLE" {
                    sub_title = items[9..].to_owned();
                }
            }
            if items.as_bytes()[0] as char == 'W' {
                if &items[..4] == "WAVE" {
                    wave = items[5..].to_owned();
                }
            }
            if &items[..] == "#START" {
                if start == 0 {
                    start = i+1;
                } else {
                    panic!("エラー：#STARTが複数回定義されています。");
                }
            }
            if &items[..] == "#END" {
                if end == 0 {
                    end = i-1;
                } else {
                    panic!("エラー：#ENDが複数回定義されています。");
                }
            }
        }

        println!("{}",title);
        println!("{}",sub_title);
        println!("{}",wave);
        if title.is_empty() { panic!("エラー:タイトルタグが定義されていません。") };
        let wave = match File::open(wave) {
            Err(e) => {
                return Err(e);
            }
            Ok(t) => t
        };

        let  score = Self::create_score(&tja,start,end);

        Ok(Self {
            title,
            sub_title,
            level: 10,
            bpm: 135,
            wave,
            songvol: 100,
            sevol: 100,
            course: Course::Oni,
        })
    }

    fn create_score(tja: &Vec<String>,start: usize,end: usize) -> Score {
        let mut measure = 1;
        let tja = &tja[start..end];
        for i in tja {
            let bytes = i.as_bytes();
            for (i,&items) in bytes.iter().enumerate() {
                if items == b',' {
                    measure += 1;
                }
            }
        }
        println!("Measure: {}",measure);

        Score {
            score_string: String::new(),
            measure
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }
}