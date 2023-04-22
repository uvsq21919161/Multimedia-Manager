use std::{path::Path, fs::File, io::Write};
use walkdir::{DirEntry, WalkDir};
use id3::{Tag, TagLike};


use crate::musicfile::{MusicFile, VideoFile};

const SUPPORTED_EXTENSIONS: [&str; 1] = ["mp3"];

/// Verifie si le fichier est compatible avec le programme. 
fn is_supported(entry: &DirEntry) -> bool {
    entry.path().is_file() &&
    SUPPORTED_EXTENSIONS.contains(&entry.path().extension().unwrap().to_str().unwrap())
}

/// Fonction qui prend en paramètre un chemin et qui retourne un Vecteur composé de plusieur ou d'une structure MusicFile.
/// 
/// Le code, parcours de façon récursif, tous les fichiers du dossier, grâce aux fichiers et la biblothèque id3:Tag et TagLike pour récupérer les métadonnées et crée une structure MusicFile qui sera ajoutée dans le vecteur musicfile.
/// 
/// Une fois tous les fichiers traités, on sauvegarde les données du vecteur dans un fichier JSON.
/// # Examples
/// ```ignore
/// scan(&path)
/// ```
pub fn scan(path: &Path) -> Vec<MusicFile> {
    let mut music_files: Vec<MusicFile> = Vec::new();
    let walker = WalkDir::new(path).into_iter();
    for entry in walker {
        let entry = match entry {
            Ok(entry) => {entry},
            Err(err) => {println!("{}, le fichier n'est pas conforme.", err);
                                break },
        };
        if is_supported(&entry) {
            // Liste des métadonnées récupérés avec la biblothèque id3, pour chaque fichier du dossiers, en utilisant le chemin du fichier.
            // Dans le cas où le fichier ne possède pas de métadonnées récupéré par la bibliothèque id3, renvoi le nom du fichier et passe à l'élément suivant.
            let tag = match Tag::read_from_path(entry.path()) {
                Ok(elt) => {elt},
                Err(_pb) => {println!("{:?}, ce fichier ne possède pas de métadonnées utilisables.", entry.path());
                                    break;},
            };
            // Rajoute quelques métadonnées dans le vecteur music_files et aussi le chemin.
            music_files.push(MusicFile::new(
                entry.path(),
                tag.artist().map(|s| s.to_owned()),
                tag.album().map(|s| s.to_owned()),
                tag.title().map(|s| s.to_owned()),
                tag.year().map(|s| s.to_owned()),
                tag.duration().map(|s| s.to_owned()),
            ));
        }
    };

    // Sérialiser les données analysées, qui sera écrit dans un fichier JSON du nom sauvegarde.json, par élément dans music_files.
    let mut output = File::create("sauvegarde.json").expect("La création du fichier JSON n'as pas pu être réalisé.");
    let serialized =serde_json::to_string_pretty(&music_files).expect("La conversion en JSON n'as pas était réalisé.");
    write!(output,"{}",serialized).expect("Impossibilité d'écrire sur le fichier JSON.");
    scan2(path);
    music_files
}

const SUPPORTED_EXTENSION: [&str; 1] = ["mp4"];

/// Verifie si le fichier est compatible avec le programme. 
fn is_supported2(entry: &DirEntry) -> bool {
    entry.path().is_file() &&
    SUPPORTED_EXTENSION.contains(&entry.path().extension().unwrap().to_str().unwrap())
}

/// Fonction qui prend en paramètre un chemin et qui retourne un Vecteur composé de plusieur ou d'une structure VideoFile.
/// 
/// Le code, parcours de façon récursif, tous les fichiers du dossier, grâce aux fichiers et la biblothèque mp4ameta pour récupérer les métadonnées et crée une structure MusicFile qui sera ajoutée dans le vecteur VideoFile.
/// 
/// Une fois tous les fichiers traités, on sauvegarde les données du vecteur dans un fichier JSON.
pub fn scan2(path: &Path){
    let mut video_file: Vec<VideoFile> = Vec::new();
    let walker = WalkDir::new(path).into_iter();
    for entry in walker {
        let entry = match entry {
            Ok(entry) => {entry},
            Err(err) => {println!("{}, le fichier n'est pas conforme.", err);
                                break },
        };
        if is_supported2(&entry) {
            // Liste des métadonnées récupérés avec la biblothèque mp4ameta, pour chaque fichier du dossiers, en utilisant le chemin du fichier.
            // Dans le cas où le fichier ne possède pas de métadonnées récupéré par la bibliothèque mp4ameta, renvoi le nom du fichier et passe à l'élément suivant.
            let tag = mp4ameta::Tag::read_from_path(entry.path()).unwrap();
            // Rajoute quelques métadonnées dans le vecteur video_files et aussi le chemin.
            video_file.push(VideoFile::new(
                entry.path(),
                tag.artist().map(|s| s.to_owned()),
                tag.title().map(|s| s.to_owned()),
                tag.duration().map(|s| s.to_owned()),
                tag.genre().map(|s| s.to_owned()),
                tag.year().map(|s| s.to_owned()),
            ));
        }
    };

    // Sérialiser les données analysées, qui sera écrit dans un fichier JSON du nom sauvegarde.json, par élément dans video_files.
    let mut output = File::create("sauvegardemp4.json").expect("La création du fichier JSON n'as pas pu être réalisé.");
    let serialized =serde_json::to_string_pretty(&video_file).expect("La conversion en JSON n'as pas était réalisé.");
    write!(output,"{}",serialized).expect("Impossibilité d'écrire sur le fichier JSON.");
    
    
}
