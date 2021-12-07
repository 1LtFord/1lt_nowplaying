mod mpris_mediaplayer2;
use std::{thread, time};
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
    print_agpl_v3_disclaimer();
    print_external_librarys();

    let sleep_duration = time::Duration::from_millis(1000);
    let mut old_metadata = mpris_mediaplayer2::PlayerMetadata::new();
    let mut work_env = WorkingEnvironment::new();

    loop {
        let mediaplayers =  get_names_of_mediaplayers();
        //get playback status of detected mediaplayers
        for mediaplayer in &mediaplayers {
            let playing = match mpris_mediaplayer2::get_media_player_playback_status(&mediaplayer) {
                Ok(pbs) => pbs,
                Err(error) => panic!("Error while getting mediaplayer playback status: {:?}", error)
            };
            //Get playback metadata if mediaplayer is playing
            if playing {
                let metadata = mpris_mediaplayer2::PlayerMetadata::new();
                let metadata = match metadata.update_metadata_of_player(mediaplayer.to_string()) {
                    Ok(new_metadata) => new_metadata,
                    Err(error) => panic!("Error while getting mediaplayer metadata: {:?}", error)
                };
                if old_metadata != metadata {
                    println!("{}", mediaplayer);
                    println!("artist: {}", metadata.artist);
                    println!("title: {}", metadata.title);
                    println!("album: {}", metadata.album);
                    println!("");

                    work_env = match work_env.write_to_now_playing_file(&metadata) {
                        Ok(workenv) => workenv,
                        Err(error) => panic!("Cannot write to now_playing.txt: {:?}", error),
                    };
                    old_metadata = metadata;
                }
                break;
            } else {
                continue;
            }
        }
        thread::sleep(sleep_duration);
    }

}

fn get_names_of_mediaplayers() -> Vec<String> {
    let mediaplayers = match mpris_mediaplayer2::get_media_players() {
        Ok(vec_of_mediaplayer_strings) => vec_of_mediaplayer_strings,
        Err(error) => panic!("Error while getting mediaplayers: {:?}", error)
    };
    mediaplayers
}

pub struct WorkingEnvironment {
    pub path_work_directory: PathBuf,
    pub file_now_playing: File,
}

impl WorkingEnvironment {
    pub fn new() -> WorkingEnvironment {
        WorkingEnvironment::create_working_environment("now_playing.txt".to_string())
    }

    fn create_working_environment(now_playing_file_name: String) -> WorkingEnvironment {
        let work_directory = WorkingEnvironment::get_work_directory();

        match WorkingEnvironment::create_nowplaying_directory(&work_directory) {
            Ok(()) => {},
            Err(error) => panic!("Error while crating now_playing directory: {:?}", error)
        };

        let file = match WorkingEnvironment::create_new_nowplaying_file(&work_directory, now_playing_file_name) {
            Ok(new_file) => new_file,
            Err(error) => panic!("Error while creating now_playing.txt: {:?}", error)
        };

        WorkingEnvironment {
            path_work_directory: work_directory,
            file_now_playing: file,
        }
    }

    fn get_work_directory() -> PathBuf {
        let home_path = match env::var("HOME") {
            Ok(home_path) => home_path,
            Err(error) => panic!("No home directory set. Can't crate now_playing.txt: {:?}", error),
        };
        PathBuf::from(format!("{}/.local/share/1lt_software/1lt_nowplaying/", home_path))
    }

    fn create_nowplaying_directory(work_path: &PathBuf) -> Result<(), io::Error> {
        fs::create_dir_all(work_path)?;
        Ok(())
    }

    fn create_new_nowplaying_file(work_directory: &PathBuf, file_name: String) -> Result<File, io::Error> {
        let file = File::create(format!("{}{}", work_directory.display(), file_name))?;
        Ok(file)
    }

    pub fn write_to_now_playing_file(mut self, metadata: &mpris_mediaplayer2::PlayerMetadata) -> Result<WorkingEnvironment, io::Error> {
        let mut np_string = String::new();
        if metadata.artist != "" {
            np_string = format!("{}", metadata.artist);
        } else {}
        if metadata.title != "" {
            np_string = format!("{} - \"{}\"", np_string, metadata.title);
            if metadata.album != "" {
                np_string = format!("{} [{}]     ", np_string, metadata.album );
            } else {}
        }  else {}

        self.file_now_playing.set_len(0)?;
        self.file_now_playing.seek(SeekFrom::Start(0))?;
        self.file_now_playing.write_all(np_string.as_bytes())?;
        Ok(self)
    }
}

fn print_agpl_v3_disclaimer() {
    println!("1Lt-NowPlaying");
    println!("----------------------------------------------------------------------------");
    println!("This program is free software: you can redistribute it and/or modify");
    println!("it under the terms of the GNU Affero General Public License as published");
    println!("by the Free Software Foundation, either version 3 of the License, or");
    println!("(at your option) any later version.\n");
    println!("This program is distributed in the hope that it will be useful,");
    println!("but WITHOUT ANY WARRANTY; without even the implied warranty of");
    println!("MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the");
    println!("GNU Affero General Public License for more details.\n");
    println!("You should have received a copy of the GNU Affero General Public License");
    println!("along with this program.  If not, see https://www.gnu.org/licenses/.");
    println!("----------------------------------------------------------------------------\n");
}

fn print_external_librarys() {
    println!("This software uses external libraries:");
    println!("dbus-rs v0.9.5");
    println!("\n");
}
