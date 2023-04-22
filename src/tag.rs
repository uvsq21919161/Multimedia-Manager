use std::fs::read_to_string;
use id3::{Error, ErrorKind, Tag, TagLike, Version};
use crate::{musicfile::MusicFile};


/// Rajoute un même tag sur l'ensemble d'un dossier, en écrasant le précédent.
/// 
/// # Examples
/// Basic usage 
/// ```ignore
/// let tags : Vec<Vec<String>> = Vec::new();
/// tagadd(tags);
/// ```

pub fn tagadd(tags : Vec<Vec<String>>){
    println!("{:?}",tags);
    let data = read_to_string("sauvegarde.json").expect("pas de fichier");
    let music_file: Vec<MusicFile> =
    serde_json::from_str(&data).expect("Le JSON n'est pas sous le bon format.");
    for vec in tags{
        println!("yes");
        for elt in &music_file{
            println!("yes;s");
            let mut tagdata = match Tag::read_from_path(elt.path.clone()) {
                Ok(tag) => tag,
                Err(Error{kind: ErrorKind::NoTag, ..}) => Tag::new(),
                Err(_) => panic!("Tag non disponible"),
            };
            match vec[0].as_str() {
                "artist" => tagdata.set_artist(vec[1].clone()),
                "album" => tagdata.set_album(vec[1].clone()),
                "title" => tagdata.set_title(vec[1].clone()),
                "year" => tagdata.set_year(vec[1].clone().parse::<i32>().expect("La valeur pour l'année n'est pas un nombre")),
                "duration" => tagdata.set_duration(vec[1].clone().parse::<u32>().expect("La valeur pour la durée n'est pas un nombre")),
                &_ => {}
            }
            tagdata.write_to_path(elt.path.clone(), Version::Id3v24).expect("Erreur à l'écriture du nouveau tag");
        }
    }


}