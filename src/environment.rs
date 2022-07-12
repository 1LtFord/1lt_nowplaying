use std::fs;
use std::fs::File;
use std::io;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::mpris_mediaplayer2;
use crate::config::Config;

pub struct WorkingEnvironment {
    pub path_work_directory: PathBuf,
    pub file_now_playing: File,
    config: Config
}

impl WorkingEnvironment {
    pub fn new(config: Config) -> WorkingEnvironment {
        WorkingEnvironment::create_working_environment(config)
    }

    fn create_working_environment(config: Config) -> WorkingEnvironment {
        let nowplaying_path = PathBuf::from(config.nowplaying_path.clone());
        let work_directory = PathBuf::from(nowplaying_path.parent().unwrap());

        match WorkingEnvironment::create_nowplaying_directory(&work_directory) {
            Ok(()) => {},
            Err(error) => panic!("Error while crating now_playing directory: {:?}", error)
        };

        let file = match WorkingEnvironment::create_new_nowplaying_file(&nowplaying_path) {
            Ok(new_file) => new_file,
            Err(error) => panic!("Error while creating now_playing.txt: {:?}", error)
        };

        WorkingEnvironment {
            path_work_directory: work_directory,
            file_now_playing: file,
            config
        }
    }

    fn create_nowplaying_directory(work_path: &PathBuf) -> Result<(), io::Error> {
        fs::create_dir_all(work_path)?;
        Ok(())
    }

    fn create_new_nowplaying_file(nowplaying_path: &PathBuf) -> Result<File, io::Error> {
        let file = File::create(nowplaying_path)?;
        Ok(file)
    }

    pub fn write_to_now_playing_file(mut self, metadata: &mpris_mediaplayer2::PlayerMetadata) -> Result<WorkingEnvironment, io::Error> {
        let mut np_string = String::new();
        if metadata.artist != "" && self.config.display_artist {
            np_string = format!("{}", metadata.artist);
        }
        if metadata.title != "" && self.config.display_title {
            np_string = format!("{} - \"{}\"", np_string, metadata.title);
        }
        if metadata.album != "" && self.config.display_album {
            np_string = format!("{} [{}]", np_string, metadata.album );
        }
        if np_string.len() > 0 {
            np_string = format!("{}     ", np_string);
        }

        self.file_now_playing.set_len(0)?;
        self.file_now_playing.seek(SeekFrom::Start(0))?;
        self.file_now_playing.write_all(np_string.as_bytes())?;
        Ok(self)
    }
}