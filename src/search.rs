use crate::musicfile::{MusicFile, AnyFile};
use std::fs::read_to_string;

/// Transforme le vecteur obtenue par .search_elt(), correspondant aux arguments de recherche de la ligne de commande, en un vecteur contenant des vecteurs.
/// 
/// Cette fonction prend en argument un vec<String> et qui renvoie un Vec<Vec<String>>.
/// 
/// Chacun des sous-vecteurs correspond à un élément de la recherche, exemple : ["artiste", "nom"],["title", "titre"]]
/// 
/// Cette fonction permet également, s'il y a un élément composé de plusieurs mots, les mots sont concaténés.

pub fn readelt(val : Vec<String>) -> Vec<Vec<String>>{
    let mut vecrest : Vec<Vec<String>> = Vec::new();
    let mut argument : Vec<String> = Vec::new();
    let mut search : Vec<String> = Vec::new();
    for n in 0..(val.len()){
        if &val[n].to_lowercase() == "artist" || &val[n].to_lowercase() == "album" || &val[n].to_lowercase() == "title" || &val[n].to_lowercase() == "year" || &val[n].to_lowercase() == "genre" || &val[n].to_lowercase() == "duration"{
            argument.push(val[n].to_ascii_lowercase().clone());
        }
        else if &val[n].to_lowercase() == "and" || &val[n] == "&&" || n == (val.len()-1){
            if n == (val.len()-1) {
                search.push(val[n].clone());
                if &val[n].to_lowercase() == "and" || &val[n] == "&&"{
                    None.expect("Veuillez rajouter un élément après AND")
                }
            }
            argument.push(search.join(" "));
            search.clear();
            vecrest.push(argument.clone());            
            argument.clear();
        }
        else if &val[n] == ":"{}
        else {
            search.push(val[n].clone())
        }        
    }    
    vecrest
}

/// Renvoi un Vec<MusicFile> composé des éléments correspondant à la recherche, renvoi un message d'erreur si la recherche n'aboutit pas.
/// 
/// Fonction prenant en argument un Vec<Vec<String>>, d'un vec<MusicFile> et d'une option<&str> qui sera soit None soit Some("Json").
/// 
/// Dans le cas où il y a un Some(Json), la fonction va lire le fichier sauvegarde.json pour créer un nouveau MusicFile.
/// # Examples
/// ```ignore
/// search(elt, music_file, None);
/// ```
pub fn search(elt : Vec<Vec<String>>,mut musicfile : Vec<MusicFile>, json : Option<&str> ) -> Vec<MusicFile>{
    let mut result : Vec<MusicFile> = Vec::new();
    match json {
        Some(_) => {let data = read_to_string("sauvegarde.json").expect("pas de fichier");
                    musicfile =
                    serde_json::from_str(&data).expect("Le JSON n'est pas sous le bon format.")}
        None => {}
    }
    for file in &musicfile{
            match elt[0][0].as_str() {
                "artist" => {if elt[0][1].to_lowercase() == match file.artist(){
                    Some(x) => {x},
                    None => {continue;},
                }.to_lowercase() && !result.contains(file){
                    result.push(file.clone());
                    
                }},
                "album" => {if elt[0][1].to_lowercase() == match file.album() {
                    Some(x) => {x},
                    None => {continue;},
                }.to_lowercase() && !result.contains(file){
                        result.push(file.clone());
                    }
                },
                "title" => {if elt[0][1].to_lowercase() == match file.title() {
                    Some(x) => {x},
                    None => {continue;},
                }.to_lowercase() && !result.contains(file) {
                        result.push(file.clone());
                }},
                "year" => {if elt[0][1] == match file.year() {
                    Some(x) => {x},
                    None => {continue},
                }.to_string() && !result.contains(file) {
                        result.push(file.clone());
                }},
                "duration" => {if elt[0][1].to_lowercase() == match file.duration(){
                    Some(x) => {x},
                    None => {continue;},
                }.to_string() && !result.contains(file) {
                        result.push(file.clone());
                }},
                &_ => {}
            
        }
    }
    if elt.len() != 1{
        let mut eltrecur : Vec<Vec<String>> = Vec::new();
        for nb  in elt.iter().skip(1){
            eltrecur.push(nb.clone())
        }
        result = search(eltrecur,result,None);
    }
    if !result.is_empty() {
        result
    }
    else {
        println!("Aucun résultat trouvé dans ce fichier");
        panic!("Veuillez retenter")
    }
}

