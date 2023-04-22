use crate::musicfile::{MusicFile, AnyFile};
use markdown_gen::markdown::{Markdown, AsMarkdown, List};
use std::{fs::{File,read_to_string}, path::Path};
use chrono::Utc;

/// Créer un fichier Markdown contenant le résultat d'une requête, la fonction prend en argument, un vec<MusicFile>, un chemin pour la création du fichier, du mode qui est un &str, "JSON", et enfin d'un filtre utilisé pour la fonction search, qui est un Option<Vec<String>>.
/// 
/// Il ya plusieurs façon de l'utiliser:
/// 
///     - on peut passer par le fichier sauvegarde.json, de ce fait  :
///         - cargo run write2md JSON chemin
///       Inconvénient ne fonctionne pas si le fichier sauvegarde.json n'existe pas ou pas sous le bon format, il faudra obligatoirement faire un scan ou un search avant.
///     - ou on peut aussi faire un scan ou le search en même temps, mais sans passer par le fichier JSON, la commande sur le terminal sera :
///         - cargo run write2md Scan path_out path_in
///         - cargo run write2md Search path_out path_in catégorie : recherche
///             path_out = chemin de création de fichier; path_in = chemin à analyser.
///       Inconvénient requête longue à écrire, mais ne néccésite pas d'avoir effectué un scan ou un search au préalable.
pub fn write2md(mut music_file : Vec<MusicFile>, output : &Path, mode: &str, filter : Option<Vec<String>>){
    let file = File::create(output.with_extension("md")).expect("Impossible de créer ce fichier.");
    let mut md = Markdown::new(file);
    
    if mode == "JSON"{
        let data = read_to_string("sauvegarde.json").expect("Fichier non existent.");
        music_file =
        serde_json::from_str(&data).expect("Le JSON n'est pas sous le bon format.");
    }
    else {
        let mut entete : String = mode.to_uppercase();
        let time = Utc::now();
        entete.push_str(" du ");
        entete.push_str( &time.date_naive().to_string());
        md.write(entete.heading(1)).expect("L'écriture de l'entête sur le fichier n'as pas pu être réalisé");
    }
    if mode.to_lowercase() == "search"{
        let mut filtre : String = String::from("    Filtre = ");
        for elt in match filter {
            Some(x) => {x},
            None => {panic!("Pas d'argument.")},
        }{
            filtre.push_str(&elt);
            filtre.push(' ')
        }
        md.write(filtre.as_str()).expect("L'écriture du filtre sur le fichier n'as pas pu être réalisé");
    }
    for n in 1..(music_file.len()+1){
        let music = "Fichier MP3 ".to_string()+ &n.to_string();
        md.write(music.italic().heading(2)).expect("N'a pas pu écrire dans le fichier md");
        md.write(
            List::new(true)
                .item("Path".bold().paragraph().append(" : ").append(music_file[n-1].path.clone().into_os_string().to_str().expect("Pas un chemin")
                ))
                .item("Title".bold().paragraph().append(" : ").append(match music_file[n-1].title() {
                    Some(x) => x,
                    None => "NILL".to_string(),
                }.as_str()
                ))
                .item("Artist".bold().paragraph().append(" : ").append(match music_file[n-1].artist() {
                    Some(x) => x,
                    None => "NILL".to_string(),
                }.as_str()
                ))
                .item("Album".bold().paragraph().append(" : ").append(match music_file[n-1].album() {
                    Some(x) => x,
                    None => "NILL".to_string(),
                }.as_str()
                ))
                .item("Year".bold().paragraph().append(" : ").append(match music_file[n-1].year().map(|s| s.to_string()) {
                    Some(x) => x,
                    None => "NILL".to_string(),
                }.as_str()
                ))
                .item("Duration".bold().paragraph().append(" : ").append(match music_file[n-1].duration().map(|s| s.to_string()) {
                    Some(x) => x,
                    None => "NILL".to_string(),
                }.as_str()
                ))
            ).expect("L'écriture de la Liste MusicFile sur le fichier n'as pas pu être réalisé");
        if n+1 != music_file.len()+1{
            md.write("\n").expect("Fichier non trouvé");
        }
    }

}