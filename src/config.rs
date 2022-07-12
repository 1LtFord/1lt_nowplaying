use std::time;
use std::env;
use std::fs;
use std::path::Path;

use config_1lt::data::{config_file::ConfigFile, config_attribute::ConfigAttribute};
use config_1lt::file::{read::read_config_file, write::write_config_file};

#[derive(Clone)]
pub struct Config {
    pub nowplaying_path: String,
    pub display_artist: bool,
    pub display_album: bool,
    pub display_title: bool,
    pub sleep_duration: time::Duration,
}


impl Config {
    pub fn new() -> Config {
        let nowplaying_path = format!("{}now_playing.txt", get_default_work_directory());
        let display_artist = true;
        let display_album = true;
        let display_title = true;
        let sleep_duration = time::Duration::from_millis(1000);

        let default_config = Config {
            nowplaying_path,
            display_artist,
            display_album,
            display_title,
            sleep_duration
        };

        Config::read_config(Config::get_config_location(), default_config)
    }

    fn get_config_location() -> String {
        let home_path = match env::var("HOME") {
            Ok(home_path) => home_path,
            Err(err) => format!("No home directory set. Can't locate config file: {:?}", err)
        };
        let config_path = format!("{}/.config/1lt_software/1lt_nowplaying/", home_path);

        //Create folders if they don't exist
        if !Path::new(&config_path).is_dir() {
            match fs::create_dir_all(&config_path) {
                Ok(()) => (),
                Err(err) => panic!("Could not create config directorys: {:?}", err)
            }
        };
        
        format!("{}config", config_path)
    }   

    fn read_config(config_path: String, default_config: Config) -> Config {
        //create configfile with default values if config file does not exist
        if !Path::new(&config_path).exists() {
            Config::write_default_config(&config_path, &default_config);
            return default_config.clone()
        }
        else {
            let file = match read_config_file(config_path.clone()) {
                Ok(file) => file,
                Err(err) => panic!("Error while reading config file: {}", err)
            };

            for cfgg in file.config_groups {
                if cfgg.group_name() == "general" {
                    let nowplaying_path = match cfgg.get_config_attribute(format!("nowplaying_path")) {
                        Ok(att) => att.value,
                        Err(()) => default_config.nowplaying_path.clone()
                    };
                    let display_artist = match cfgg.get_config_attribute(format!("display_artist")) {
                        Ok(att) => att.value.parse::<bool>().unwrap(),
                        Err(()) => default_config.display_artist.clone()
                    };
                    let display_album = match cfgg.get_config_attribute(format!("display_album")) {
                        Ok(att) => att.value.parse::<bool>().unwrap(),
                        Err(()) => default_config.display_album.clone()
                    };
                    let display_title = match cfgg.get_config_attribute(format!("display_title")) {
                        Ok(att) => att.value.parse::<bool>().unwrap(),
                        Err(()) => default_config.display_title.clone()
                    };
                    let sleep_duration = match cfgg.get_config_attribute(format!("sleep_duration")) {
                        Ok(att) => time::Duration::from_millis(att.value.parse::<u64>().unwrap()),
                        Err(()) => default_config.sleep_duration.clone()
                    };
                    
                    return Config {
                        nowplaying_path,
                        display_artist,
                        display_album,
                        display_title,
                        sleep_duration
                    }
                }
            }
            return default_config.clone()
        }
    }

    #[allow(unused_must_use)]
    fn write_default_config(config_path: &String, default_config: &Config) {
        let mut new_config: ConfigFile = ConfigFile::new(config_path.clone());
        
        new_config.add_config_group(format!("general"));
        new_config.config_groups[0].add_config_attribute(ConfigAttribute::new(format!("nowplaying_path"), default_config.nowplaying_path.clone()).unwrap());
        new_config.config_groups[0].add_config_attribute(ConfigAttribute::new(format!("display_artist"), format!("{}", default_config.display_artist)).unwrap());
        new_config.config_groups[0].add_config_attribute(ConfigAttribute::new(format!("display_album"), format!("{}", default_config.display_album)).unwrap());
        new_config.config_groups[0].add_config_attribute(ConfigAttribute::new(format!("display_title"), format!("{}", default_config.display_title)).unwrap());
        new_config.config_groups[0].add_config_attribute(ConfigAttribute::new(format!("sleep_duration"), default_config.sleep_duration.as_millis().to_string()).unwrap());
        
        match write_config_file(&new_config) {
            Ok(()) => println!("No config file found! Created a new one at {} \n", config_path),
            Err(err) => panic!("Could not write config file: {}", err)
        }
    }
}


pub fn get_default_work_directory() -> String {
    let home_path = match env::var("HOME") {
        Ok(home_path) => home_path,
        Err(err) => panic!("No home directory set. Can't crate now_playing.txt: {:?}", err),
    };
    format!("{}/.local/share/1lt_software/1lt_nowplaying/", home_path)
}