use std::{thread, time};
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};

use crate::web_display::WebDisplay;
use crate::WorkingEnvironment;
use crate::mpris_mediaplayer2;
use crate::config::Config;


pub struct Service {
    sleep_duration: time::Duration,
    work_env: WorkingEnvironment,
    display_artist: bool,
    display_album: bool,
    display_title: bool,
    web_display: WebDisplay
}

pub struct NowplayingData {
    pub current_title: String,
    pub current_artist: String,
    pub current_album: String
}

impl Service {
    pub fn new(config: &Config, work_env: WorkingEnvironment) -> Service {
        let sleep_duration = config.sleep_duration;
        let display_artist = config.display_artist;
        let display_album = config.display_album;
        let display_title = config.display_title;
        let web_display = WebDisplay::new(config.port.clone(), config.web_files_text.clone(), config.public.clone());
        Service {
            sleep_duration,
            work_env,
            display_artist,
            display_album,
            display_title,
            web_display
        }
    }

    pub fn start(self) {
        self.now_playing()
    }

    fn now_playing(mut self) {
        let mut old_metadata = mpris_mediaplayer2::PlayerMetadata::new();
        let (tx, rx): (Sender<NowplayingData>, Receiver<NowplayingData>) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let rx_web = Arc::clone(&rx);
        thread::spawn(move || self.web_display.start(rx_web));

        loop {
            let mediaplayers =  Service::get_names_of_mediaplayers();
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
                        let i = rx.lock().unwrap().try_recv();
                        
                        match tx.send(NowplayingData{current_artist: metadata.artist.clone(), current_title: metadata.title.clone(), current_album: metadata.album.clone()}) {
                            Ok(()) => (),
                            Err(error) => panic!("cannot send data to http service thread: {error}")
                        };

                        println!("{}", mediaplayer);
                        
                        if self.display_artist {
                            println!("artist: {}", metadata.artist);
                        } else {
                            println!("artist (hidden): {}", metadata.artist);
                        }
                        
                        if self.display_title {
                            println!("title: {}", metadata.title);
                        } else {
                            println!("title (hidden): {}", metadata.title);
                        }

                        if self.display_album {
                            println!("album: {}", metadata.album);
                        } else {
                            println!("album (hidden): {}", metadata.album);
                        }
                        println!("");
    
                        self.work_env = match self.work_env.write_to_now_playing_file(&metadata) {
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
            thread::sleep(self.sleep_duration);
        }
    }

    fn get_names_of_mediaplayers() -> Vec<String> {
        let mediaplayers = match mpris_mediaplayer2::get_media_players() {
            Ok(vec_of_mediaplayer_strings) => vec_of_mediaplayer_strings,
            Err(error) => panic!("Error while getting mediaplayers: {:?}", error)
        };
        mediaplayers
    }
}