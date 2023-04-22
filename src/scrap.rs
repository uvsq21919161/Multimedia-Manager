use crate::musicfile::{MusicFile, AnyFile};
use serde_json::Value;
use id3::{Error, ErrorKind, Tag, TagLike, Version};
use mp3_duration::from_path;


///Scrap permet de chercher des données supplémentaire sur musicbrainz + changement des tags si la music est dans le répertoire.
/// # Examples
/// ```ignore
/// scrap(music_files)
/// ```
#[tokio::main]
pub async fn scrap(music_files: &Vec<MusicFile>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nRécupération de données depuis la base de données de musicbrainz.org.");
    for music in music_files{
        let artist = music.artist().expect("Pas d'artiste, veuillez rajouter au minimum l'artiste et le titre via la commande tag et scan pour la musique.");
        let title = music.title().expect("Pas de titre, veuillez rajouter au minimum l'artiste et le titre via la commande tag et scan pour la musique.");
        let client = reqwest::Client::builder()
            .user_agent("MyAwesomeTagger/1.2.0")
            .build()?;
        let url = "https://musicbrainz.org/ws/2/recording/?query=recording:".to_owned()
            + &title
            + " AND artist:"
            + &artist
            + "&limit=1&fmt=json";
        println!("{}",url);
        let result = client.get(&url).send().await?.text().await?;
        let v: Value = serde_json::from_str(&result)?;

        let album = v["recordings"][0]["releases"][0]["title"].to_string()
            .trim_matches('\"').to_string();

        let year = v["recordings"][0]["first-release-date"].to_string()
            .trim_matches('\"').split('-')
            .next()
            .expect("Pas de données suivantes.")
            .parse::<i32>().expect("Echec de la conversion.");
        
        let duration = from_path(music.path.clone()).expect("Chemin incorrect.").as_secs_f32() as u32 ;
        let mut tagdata = match Tag::read_from_path(music.path.clone()) {
            Ok(tag) => tag,
            Err(Error{kind: ErrorKind::NoTag, ..}) => Tag::new(),
            Err(_) => panic!("Tag non disponible"),
        };
        tagdata.set_album(album);
        tagdata.set_year(year);
        tagdata.set_duration(duration);
        tagdata.write_to_path(music.path.clone(), Version::Id3v24).expect("Erreur à l'écriture du nouveau tag");
        println!(
            "\nLes tags ont était remplacé par les nouveaux, pour {:?}.\n",title);  
    }
    Ok(())
}
