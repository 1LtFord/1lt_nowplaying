use dbus::{blocking::Connection, arg};
use std::collections::HashMap;
use std::time::Duration;

#[derive(PartialEq)]
pub struct PlayerMetadata {
    pub artist: String,
    pub title: String,
    pub album: String,
}

impl PlayerMetadata {
    pub fn new() -> PlayerMetadata {
        PlayerMetadata {
            artist: String::new(),
            title: String::new(),
            album: String::new(),
        }
    }
    pub fn update_metadata_of_player(mut self, mediaplayer: String) -> Result<PlayerMetadata, Box<dyn std::error::Error>> {
        //Connect to D-Bus and set operating location
        let session = Connection::new_session()?;
        let player_name = format!("org.mpris.MediaPlayer2.{}", mediaplayer);
        let proxy = session.with_proxy(player_name, "/org/mpris/MediaPlayer2", Duration::from_millis(5000));

        //get Mediaplayer Metadata
        use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
        let metadata: HashMap<String, arg::Variant<Box<dyn arg::RefArg>>> = proxy.get("org.mpris.MediaPlayer2.Player", "Metadata")?;

        //Get Artist(s)
        if metadata.contains_key("xesam:artist") {
            let artists = metadata["xesam:artist"].0.as_iter();
            if artists.is_some() {
                let artists = artists.unwrap();
                for artist in artists {
                    let artist = artist.as_str().unwrap();
                    self.artist = format!("{}{} ", self.artist, artist);
                }
            } else {
                let artist = &metadata["xesam:artist"].0;
                let artist = artist.as_str().unwrap();
                self.artist = format!("{}", artist);
            }
        }

        //Get Title
        if metadata.contains_key("xesam:title") {
            let title = &metadata["xesam:title"].0;
            let title = title.as_str().unwrap();
            self.title = format!("{}", title);
        }

        //Get Album
        if metadata.contains_key("xesam:album") {
            let album = &metadata["xesam:album"].0;
            let album = album.as_str().unwrap();
            self.album = format!("{}", album);
        }

        Ok(self)
    }
}

pub fn get_media_players() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    //Connect to D-Bus and set operating location
    let session = Connection::new_session()?;
    let proxy = session.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));
    //Get List of all registered names
    let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;

    // Only get mediaplayers out of registered names
    let mut mediaplayers = Vec::new();
    for name in names {
        if name.contains("org.mpris.MediaPlayer2.") {
            mediaplayers.push(name.replace("org.mpris.MediaPlayer2.",""))
        }
    }

    Ok(mediaplayers)
}

pub fn get_media_player_playback_status(mediaplayer: &String) -> Result<bool, Box<dyn std::error::Error>> {
    let session = Connection::new_session()?;
    let player_name = format!("org.mpris.MediaPlayer2.{}", mediaplayer);
    let proxy = session.with_proxy(player_name, "/org/mpris/MediaPlayer2", Duration::from_millis(5000));

    //get mediaplayer playback status
    use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
    let playback_status: String = proxy.get("org.mpris.MediaPlayer2.Player", "PlaybackStatus")?;

    //println!("{}", playback_status);
    if playback_status == "Playing"{
        Ok(true)
    } else {
        Ok(false)
    }
}
