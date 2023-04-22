use std::{io::{stdin,Write}, fs::File};

use crate::{scan::scan, search::{search, readelt}, write2md::write2md, musicfile::MusicFile, writeplaylist::writeplaylist};

/// Fonction pour aider l'utilisateur si aucun argument est donner.
/// # Examples
/// ```ignore
/// interaction()
/// ```
pub fn user_helper() {
    let mut requete = String::new();
    let mut json = String::new();
    let mut option = String::new();
    let mut path = String::new();
    let mut category = String::new();
    let mut arguments = String::new();


    println!("\nBienvenue dans le mode intéractif du programme medman.\n");
    println!("scan : Permet d'analyser tous les musiques d'un répertoire, et les enregistre dans
    un fichier JSON. \n");

    println!("search : Permet de trier un scan ou un fichier JSON
    avec différents argument ( year / title / artist / album / duration) et differents
    modificateur (and / or).\n");

    println!("write2md : Crée un markdown avec le chemin indiqué, contenant soit le résultat d'un scan ou 
    d'un search ou le contenu d'un fichier JSON.\n");

    println!("writeplaylist : Crée une playlist avec le chemin indiqué, contenant le contenu d'un fichier JSON.\n");

    println!("Veuillez entre la commande pour continuer : ");
   
    'help: loop {
        let _ = stdin().read_line(&mut requete);
        match requete.as_str().trim() {
            "scan" => {
                println!("Quelle répertoire souhaitez-vous scanner ?");
                stdin().read_line(&mut path).expect("Chemin non reconnu");
                let path = std::path::Path::new(&path[0..path.len()-2]);            
                let music_files = scan(path);
                for music_file in &music_files {
                    println!("{:?}", music_file);
                };
                break 'help;},

            "search" => {
                println!("Souhaitez vous utiliser un fichier serialise/JSON ? yes/no");
                let _ = stdin().read_line(&mut json);
                let mut args_vec: Vec<String> = Vec::new();
                'search : loop {
                    println!("Ecrivez la categorie de la recherche : (year / artist / album / title / duration");
                    let _ = stdin().read_line(&mut category);
                    println!("Ecrivez l'objet de la recherche");
                    let _ = stdin().read_line(&mut arguments);
                    args_vec.push(category.as_str().trim().to_string().clone());
                    //args_vec.push(":".to_string());
                    let arg : Vec<&str> = arguments.trim().split(' ').collect();
                    for elt in arg {
                        args_vec.push(elt.to_string());
                    }
                    println!("Avez vous un autre argument ? yes/no");
                    let _ = stdin().read_line(&mut option);
                    match option.as_str().trim() {
                        "yes" =>{
                            args_vec.push("AND".to_string());
                            arguments.clear();
                            category.clear();
                            option.clear();
                        },
                        "no" => {
                            match json.as_str().trim() {
                                "yes" => {
                                    println!("{:?}",args_vec);
                                    let musicfile : Vec<MusicFile> = Vec::new();
                                    let music_files = search(readelt(args_vec), musicfile, Some("JSON"));
                                    for music_file in &music_files {
                                        println!("{:?}", music_file);
                                    };
                                    let mut output = File::create("sauvegarde.json").expect("La création du fichier JSON n'as pas pu être réalisé.");
                                    let serialized =serde_json::to_string_pretty(&music_files).expect("La conversion en JSON n'as pas était réalisé.");
                                    write!(output,"{}",serialized).expect("Impossibilité d'écrire sur le fichier JSON.");
                                    break 'search;
                                }
                                "no" => {
                                    println!("Saisir un répertoire où réaliser la recherche.");
                                    stdin().read_line(&mut path).expect("Chemin non reconnu");
                                    let path2 = std::path::Path::new(&path[0..path.len()-2]);
                                    let musicfile = scan(path2);
                                    let music_files = search(readelt(args_vec), musicfile, None);
                                    for music_file in &music_files {
                                        println!("{:?}", music_file);
                                    };
                                    let mut output = File::create("sauvegarde.json").expect("La création du fichier JSON n'as pas pu être réalisé.");
                                    let serialized =serde_json::to_string_pretty(&music_files).expect("La conversion en JSON n'as pas était réalisé.");
                                    write!(output,"{}",serialized).expect("Impossibilité d'écrire sur le fichier JSON.");
                                    break 'search;
                                } 
                                _ => {break 'search;}
                            }
                        
                        }
                        _  => {break 'help;}
                        }
                    }break 'help;
                } 
            "write2md" => {
                    println!("Saisir un chemin pour créer le fichier.");
                    stdin().read_line(&mut path).expect("Chemin non reconnu");
                    let path = std::path::Path::new(&path[0..path.len()-2]);
                    let music_file : Vec<MusicFile> = Vec::new();
                    write2md(music_file, path, "JSON", None);
            break 'help;}
            "writeplaylist" => {
                            println!("Saisir un chemin pour créer la playlist.");
                            stdin().read_line(&mut path).expect("Chemin non reconnu");
                            let path = std::path::Path::new(&path[0..path.len()-2]);
                            writeplaylist(None, path, Some("JSON"));
            break 'help;},
            _ => {},
        }
        requete.clear();
        
    };
}