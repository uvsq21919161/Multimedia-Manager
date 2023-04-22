use std::path::Path;
use m3u::{path_entry,Writer, EntryExt};
use std::fs::{File,read_to_string};
use std::env::current_dir;
use mp3_duration::from_path;
use crate::musicfile::{MusicFile, AnyFile};

/// Créer un fichier m3u créant une playlist le résultat d'une requête, la fonction prend en argument, un Option<vec<MusicFile>>, un chemin pour la création du fichier, du mode qui est un &str, "JSON".
///
/// Il ya plusieurs façon de l'utiliser:
/// 
///     - on peut passer par le fichier sauvegarde.json, de ce fait  :
///         cargo run writeplaylistJSON chemin.
///       Inconvénient ne fonctionne pas si le fichier sauvegarde.json n'existe pas ou pas sous le bon format, il faudra obligatoirement faire un scan ou un search avant.
///     - ou on peut aussi faire un scan ou le search en même temps, mais sans passer par le fichier JSON, la commande sur le terminal sera :
///         - cargo run writeplaylist Scan path_out path_in
///         - cargo run writeplaylist Search path_out path_in catégorie : recherche
///             path_out = chemin de création de fichier; path_in = chemin à analyser.
///       Inconvénient requête longue à écrire, mais ne néccésite pas d'avoir effectué un scan ou un search au préalable.
pub fn writeplaylist(musicfile : Option<Vec<MusicFile>>, output : &Path, mode : Option<&str>) {
    let music_file = match mode {
        None => { musicfile.expect("Le repertoire contient des fichiers non supporté")},
        Some(&_) => {let data = read_to_string("sauvegarde.json").expect("pas de fichier");
                    let music_file: Vec<MusicFile> =
                    serde_json::from_str(&data).expect("Le JSON n'est pas sous le bon format.");
                    music_file}
    };
    let mut playlist : Vec<EntryExt> = Vec::new();
    for elt in music_file {
        let mut info = String::from(" ");
        info.push_str(&(elt.artist().expect("Pas un nom d'artiste.")));
        info.push_str(" - ");
        info.push_str(&elt.title().expect("Pas un nom de titre."));
        let chemin = path_entry(match current_dir() {
            Ok(x) => {x},
            Err(_) => {panic!("Chemin non existent")},
        }.join(elt.path.clone())).extend(from_path(elt.path.clone()).expect("Chemin incorrect.").as_secs_f32() as u32 as f64, info);
        playlist.push(chemin);
    }

    let mut file = File::create(output.with_extension("m3u")).expect("La création de la playlist n'as pas pu aboutir, veuillez donner un chemin correct.");
    let mut file_writed = Writer::new_ext(&mut file).expect("La création de la playlist n'as pas pu aboutir.");
    for entry in &playlist {
        file_writed.write_entry(entry).expect("Le chemin n'est pas dans le bon format.")
    }
}
