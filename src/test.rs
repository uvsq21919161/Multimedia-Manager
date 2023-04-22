use chrono::Utc;
use crate::musicfile::MusicFile;
use crate::scan::scan;
use crate::search::{readelt, search};
use crate::write2md::write2md;
use crate::writeplaylist::writeplaylist;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};


#[test]
fn scantest() {
    let path = PathBuf::from("src/test/music");
    let test = scan(&path);
    let artist = String::from("Steven O'Brien");
    let title = String::from("Ambient I");
    let musicpath = Path::new("src/test/music\\Ambient-I.mp3");
    let musicpath2 = Path::new("src/test/music\\Dinos_Simyaci.mp3");
    let artist2 = String::from("Dinos");
    let album2 = String::from("Hiver à Paris");
    let title2 = String::from("Simyaci");
    let test1 = MusicFile::new(musicpath,Some(artist),None,Some(title),None,None);
    let test2 = MusicFile::new(musicpath2,Some(artist2),Some(album2),Some(title2),Some(2022),None);
    let mut testaeq: Vec<MusicFile> = Vec::new();
    testaeq.push(test1);
    testaeq.push(test2);
    let test = &test;
    assert_eq!(*test,testaeq);

}

#[test]
fn searchtest2() {
    let mut testsearch : Vec<String> = Vec::new();
    testsearch.push("artist".to_string());
    testsearch.push(":".to_string());
    testsearch.push("Jack".to_string());
    testsearch.push("Harlow".to_string());
    testsearch.push("AND".to_string());
    testsearch.push("artist".to_string());
    testsearch.push(":".to_string());
    testsearch.push("Dinos".to_string());
    let test = readelt(testsearch);
    let mut elt : Vec<String> = Vec::new();
    elt.push("artist".to_string());
    elt.push("Jack Harlow".to_string());
    let mut elt2 : Vec<String> = Vec::new();
    elt2.push("artist".to_string());
    elt2.push("Dinos".to_string());
    let mut testdefini : Vec<Vec<String>> = Vec::new();
    testdefini.push(elt);
    testdefini.push(elt2);
    assert_eq!(test,testdefini)
}

#[test]
fn searchtest() {
    let mut testsearch : Vec<String> = Vec::new();
    let path = PathBuf::from("src/test/music");
    testsearch.push("title".to_string());
    testsearch.push(":".to_string());
    testsearch.push("Ambient".to_string());
    testsearch.push("I".to_string());
    testsearch.push("AND".to_string());
    testsearch.push("artist".to_string());
    testsearch.push(":".to_string());
    testsearch.push("Steven".to_string());
    testsearch.push("O'Brien".to_string());
    let testfile = scan(&path);
    let testsearchelt = readelt(testsearch);
    let testresult = search(testsearchelt,testfile,None);
    let artist = String::from("Steven O'Brien");
    let title = String::from("Ambient I");
    let musicpath = Path::new("src/test/music\\Ambient-I.mp3");
    let testx = MusicFile::new(musicpath,Some(artist),None,Some(title),None,None);
    let mut testaeq: Vec<MusicFile> = Vec::new();
    testaeq.push(testx);
    assert_eq!(testresult,testaeq)
}

#[test]
fn write2mdtest() {
    let path = PathBuf::from("src/test/music");
    let path_out = PathBuf::from("src/test/markdown/test");
    write2md(scan(&path), &path_out, "scan", None);
    let datatest = read_to_string(path_out.with_extension("md")).expect("Chemin inexistant");
    let mut data = String::new();
    let time = Utc::now();
    data.push_str("# SCAN du ");
    let time =  time.date_naive().to_string();
    let time : Vec<&str> = time.split("-").collect();
    let time = time.join("\\-");
    data.push_str(&time);
    data.push_str("\n## *Fichier MP3 1*\n\n   1. **Path** : src/test/music\\\\Ambient\\-I\\.mp3\n   1. **Title** : Ambient I\n   1. **Artist** : Steven O'Brien\n   1. **Album** : NILL\n   1. **Year** : NILL\n   1. **Duration** : NILL\n\n\n## *Fichier MP3 2*\n\n   1. **Path** : src/test/music\\\\Dinos\\_Simyaci\\.mp3\n   1. **Title** : Simyaci\n   1. **Artist** : Dinos\n   1. **Album** : Hiver à Paris\n   1. **Year** : 2022\n   1. **Duration** : NILL");
    assert_eq!(datatest,data)
}

#[test]
fn writeplaylisttest() {
    let path = PathBuf::from("src/test/music");
    let path_out = PathBuf::from("src/test/playlist/test");
    writeplaylist(Some(scan(&path)), &path_out, None);
    let datatest = read_to_string(path_out.with_extension("m3u")).expect("Chemin inexistant");
    let data = String::from("#EXTM3U\n#EXTINF:273, Steven O'Brien - Ambient I\nC:\\Users\\Thanu\\Desktop\\Fac\\lourd\\dmn-uvsq21919161\\Projet\\projet-medman-uvsq21919161\\src/test/music\\Ambient-I.mp3\n#EXTINF:228, Dinos - Simyaci\nC:\\Users\\Thanu\\Desktop\\Fac\\lourd\\dmn-uvsq21919161\\Projet\\projet-medman-uvsq21919161\\src/test/music\\Dinos_Simyaci.mp3\n");
    assert_eq!(datatest,data)
}
