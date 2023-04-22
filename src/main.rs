use medman::cli::CliArguments;
use medman::interact::user_helper;
use medman::musicfile::MusicFile;
use medman::scan::scan;
use medman::scrap::scrap;
use medman::search::{readelt, search};
use medman::tag::tagadd;
use medman::write2md::write2md;
use medman::writeplaylist::writeplaylist;
use std::{fs::File, io::{Write}};


fn main() {
    // Afficher la ligne de commande sous une structure CliArguments.
    let args = CliArguments::new();
    println!("{:?}", args);
    match args.command {
        Some(_) => {
            // Partie Scan.
            if args.command() == "Scan" {
                let music_files = scan(args.path());
                for music_file in &music_files {
                    println!("{:?}", music_file);
                };
            }
            
            // Partie Search, selon si on passe par un JSON ou pas.
            else if args.command() == "Search" {
                let elt = args.search_elt();
                if elt.is_empty() || elt.len() == 1 || elt.len() == 2 {
                    panic!("Veuillez suivre le format du search, utilisez -h")
                }
                else if elt[0].to_uppercase() == "JSON"{
                    let mut json : Vec<String> = Vec::new();
                    json.extend_from_slice(&elt[1..]);
                    let music_file : Vec<MusicFile> = Vec::new();
                    let json = readelt(json);
                    let res =  search(json,music_file,Some("JSON"));
                    for music_file in &res {
                        println!("{:?}", music_file);
                    };
                    let mut output = File::create("sauvegarde.json").expect("La création du fichier JSON n'as pas pu être réalisé.");
                    let serialized =serde_json::to_string_pretty(&res).expect("La conversion en JSON n'as pas était réalisé.");
                    write!(output,"{}",serialized).expect("Impossibilité d'écrire sur le fichier JSON.");
                }
                else {
                    let elt = readelt(elt);
                    let res = search(elt,scan(args.path()),None);
                    for music_file in &res {
                        println!("{:?}", music_file);
                    };
                    let mut output = File::create("sauvegarde.json").expect("La création du fichier JSON n'as pas pu être réalisé.");
                    let serialized =serde_json::to_string_pretty(&res).expect("La conversion en JSON n'as pas était réalisé.");
                    write!(output,"{}",serialized).expect("Impossibilité d'écrire sur le fichier JSON.");
                    }
                
            }
        
            // Partie Write2md, selon si on passe par un JSON ou un scan ou un search. 
            else if args.command() == "Write2mdScan" || args.command() == "Write2mdSearch" || args.command() == "Write2md JSON"{
                match args.command() {
                    "Write2mdScan" => {let music_files = scan(args.path());
                                        write2md(music_files,args.md_path_out(),"scan", None)},
                    "Write2mdSearch" => {let elt = args.search_elt();
                                        if elt.is_empty() || elt.len() == 1 || elt.len() == 2 {
                                            panic!("Veuillez suivre le format du search pour utiliser le Write2md search, utilisez -h")
                                        } 
                                        let elt1 = readelt(elt.clone()); 
                                        let music_files = search(elt1,scan(args.path()),None);
                                        write2md(music_files,args.md_path_out(),"search",Some(elt))},
                    &_ => { let music_files : Vec<MusicFile> = Vec::new();
                        write2md(music_files ,args.md_path_out(),"JSON", None)}
                }
            }
        
            //Partie Writeplaylist, selon si on passe par un JSON ou un scan ou un search.
            else if args.command() == "WriteplaylistScan" || args.command() == "WriteplaylistSearch" || args.command() == "Playlist JSON" {
                match args.command() {
                    "WriteplaylistScan" => {let music_files = scan(args.path());
                                        writeplaylist(Some(music_files),args.md_path_out(),Some("JSON"))},
                    "WriteplaylistSearch" => {let elt = args.search_elt();
                                        if elt.is_empty() || elt.len() == 1 || elt.len() == 2 {
                                            panic!("Veuillez suivre le format du search pour utiliser le Write2md search, utilisez -h")
                                        } 
                                        let elt1 = readelt(elt); 
                                        let music_files = search(elt1,scan(args.path()),None);
                                        writeplaylist(Some(music_files),args.md_path_out(),None)},
                    &_ => { writeplaylist(None,args.md_path_out(),Some("JSON"))},
                }
            }
            else if args.command() == "Tag"{
                let tagargs = readelt(args.search_elt());
                tagadd(tagargs);
            }
            else if args.command() == "Scrap" {
                let music_files = scan(args.path());
                scrap(&music_files).unwrap();
            }
            
        },
        None => {
            user_helper()},
    }

}