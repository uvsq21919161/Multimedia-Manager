use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use std::time::Duration;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(PartialEq,Eq)]
pub struct MusicFile {
    pub path: PathBuf,
    title: Option<String>,
    artist : Option<String>,
    album : Option<String>,
    year : Option<i32>,
    duration : Option<u32>
}

impl MusicFile {
    /// Créer une structure MusicFile avec des éléments donnés.
    pub fn new(path: &Path, artist: Option<String>, album: Option<String>, title : Option<String>, year:Option<i32>, duration : Option<u32> ) -> MusicFile {
        MusicFile {
            path: path.to_path_buf(),
            artist,
            album,
            title,
            year,
            duration,
        }
    }

    /// Renvoi l'album d'un MusicFile
    pub fn album(&self) -> Option<String> {
        self.album.clone()
    }
    
    /// Renvoi l'année d'un MusicFile
    pub fn year(&self) -> Option<i32> {
        self.year
    }
    
    /// Renvoi la durée d'un MusicFile
    pub fn duration(&self) -> Option<u32> {
        self.duration
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct VideoFile {
    pub path: PathBuf,
    pub artist: Option<String>,
    pub title: Option<String>,
    pub duration: Option<Duration>,
    pub genre: Option<String>,
    pub year: Option<String>,
}

impl VideoFile {
    /// Créer une structure VideoFile avec des éléments donnés.
    pub fn new(path: &Path, artist: Option<String>,  title : Option<String>, duration : Option<Duration>, genre: Option<String>,  year : Option<String> ) -> VideoFile {
        VideoFile {
            path: path.to_path_buf(),
            artist,
            title,
            duration,
            genre,
            year,
        }
    }
    
    /// Renvoi l'album d'un VideoFile
    pub fn duration(&self) -> Option<Duration> {
        self.duration
    }
    
    /// Renvoi l'année d'un VideoFile
    pub fn genre(&self) -> Option<String> {
        self.genre.clone()
    }
    
    /// Renvoi la durée d'un VideoFile
    pub fn year(&self) -> Option<String> {
        self.year.clone()
    }
}

/// Instance qui sera relié à MusicFile et VideoFile, il vont hériter de ses propriétés et de ses fonction.
pub trait AnyFile {
    type Instance;
    fn artist(&self)->Option<String>;
    fn title(&self) -> Option<String>;
}

impl AnyFile for MusicFile {
    type Instance = MusicFile;
    fn artist(&self)->Option<String>{
        self.artist.clone()
    }
    fn title(&self) -> Option<String>{
        self.title.clone()
    }
    
}

impl AnyFile for VideoFile {
    type Instance = VideoFile;

    /// Renvoi l'artiste d'une instance.
    fn artist(&self)->Option<String>{
        self.artist.clone()
    }
    /// Renvoi le titre d'une instance.
    fn title(&self) -> Option<String>{
        self.title.clone()
    }
}