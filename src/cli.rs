use structopt::StructOpt;

#[derive(StructOpt,Debug)]
/// Ensemble des actions que l'utilisateur peut effectuer.
pub enum Action {
    /// Lance une analyse sur un dossier.
    Scan{
        /// Chemin où trouver les fichiers à analyser
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    /// Lance une recherche sur un dossier.
    Search{
        /// Chemin où trouver les fichiers à analyser
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
        /// Argument pour filtrer la recherche.
        /// Sous le format: artiste : valeur OU artiste : valeur AND album : valeur.
        /// Si le premier argument est JSON effectura la recherche sur sauvegarde.json, dans ce cas mettre - pour path.
        #[structopt(name = "ARGUMENTS")]
        rest: Vec<String>,
    },
    /// Ecrit le résultat d'une requête dans un markdown
    Write2md{
        /// La requête à copier.
        request : String,
        // Chemin où sera crée le fichiers md.
        #[structopt(parse(from_os_str))]
        path_out : std::path::PathBuf,
        /// Chemin où trouver les fichiers à analyser.
        path_in : Option<std::path::PathBuf>,
        /// Argument pour filtrer la recherche.
        /// Sous le format: artiste : valeur OU artiste : valeur AND album : valeur.
        #[structopt(name = "ARGUMENTS")]
        rest: Vec<String>
    },
    /// Crée une playlist avec le résultat d'une requête sans passer par le json.
    Writeplaylist{
        /// requête à copier.
        request : String,
        /// Chemin où sera crée le fichiers m3u, qui sont des playlist.
        #[structopt(parse(from_os_str))]
        path_out : std::path::PathBuf,
        /// Chemin où trouver les fichiers à analyser.
        path_in : std::path::PathBuf,
        /// Argument pour filtrer la recherche.
        /// Sous le format: artiste : valeur OU artiste : valeur AND album : valeur.
        #[structopt(name = "ARGUMENTS")]
        rest: Vec<String>
    },
    /// Crée une playlist avec le résultat d'une requête en passant par un json.
    Writeplaylistjson{
        /// Chemin où sera crée les fichiers m3u, qui sont des playlist.
        #[structopt(parse(from_os_str))]
        path_out : std::path::PathBuf,
    },
    /// Rajouter des tags sur la dernière requête.
    Tag{
        ///Tag à rajouter, sous le format:
        ///     - catégorie : objet de la recherche AND catégorie : objet
        #[structopt(name = "ARGUMENTS")]
        elt: Vec<String> 
    },
    /// Rajoute les tags manquants des fichiers d'un répertoire.
    Scrap{
        /// Chemin où trouver les fichiers.
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    }
}


/// Représente les arguments en paramètres de ligne de commande
#[derive(Debug)]
#[derive(StructOpt)]
pub struct CliArguments {
    /// Commande à exécuter
    #[structopt(subcommand)]
    pub command: Option<Action>,
}

impl Default for CliArguments {
    fn default() -> Self {
        Self::new()
    }
}

impl CliArguments {

    /// Crée une nouvelle structure CliArguments avec la commande sur le terminal.
    pub fn new() -> CliArguments {
        CliArguments::from_args()
    }

    /// Renvoi le chemin donné par l'utilisateur.
    pub fn path(&self) -> &std::path::Path {
        match match &self.command {
            Some(x) => x,
            None => {panic!("Aucune commande est saisie")},
        } {
            Action::Scan{path} => path,
            Action::Search{path, rest:_} => path,
            Action::Write2md { request:_, path_out:_, path_in, rest:_ } => match path_in {
                        Some(x) => {x},
                        None => {panic!("Fonction path ne fonctionne pas pour le write JSON")},
            },
            Action::Writeplaylist { request:_, path_out:_, path_in, rest:_ } => path_in,
            Action::Writeplaylistjson { path_out : _ } => panic!("Write depuis un JSON ne nécessite pas un chemin."),
            Action::Tag { elt:_ } => panic!("Fonction path ne fonctionne pas pour Tag"),
            Action::Scrap { path } => path,
        }
    }

    /// Renvoi l'action demandée par l'utilisateur.
    pub fn command(&self) -> &str {
        match match &self.command {
            Some(x) => x,
            None => {panic!("Aucune commande est saisie")},
        } {
            Action::Scan{path:_} => "Scan",
            Action::Search{path:_, rest:_} => "Search",
            Action::Write2md { request, path_out:_, path_in:_, rest:_ } => {if request.to_lowercase() == "scan"{
                                                                    "Write2mdScan"
                                                                    }
                                                                    else if request.to_lowercase() == "search"{
                                                                    "Write2mdSearch"
                                                                    }
                                                                    else if request.to_lowercase() == "json"{
                                                                        "Write2md JSON"
                                                                    }
                                                                    else{
                                                                        panic!("La requête n'est pas dans les champs des possibilités, veuillez choisir entre Scan ou Search")
                                                                    }},
            Action::Writeplaylist { request, path_out:_, path_in:_, rest:_ } => {if request.to_lowercase() == "scan"{
                                                                    "WriteplaylistScan"
                                                                    }
                                                                    else if request.to_lowercase() == "search"{
                                                                    "WriteplaylistSearch"
                                                                    }
                                                                    else{
                                                                        panic!("La requête n'est pas dans les champs des possibilités, veuillez choisir entre Scan ou Search")
                                                                    }}
            Action::Writeplaylistjson { path_out : _ } => "Playlist JSON",
            Action::Tag { elt:_ } => "Tag",
            Action::Scrap { path: _ } => "Scrap",

        }
    }

    /// Renvoi le filtre d'un search.
    pub fn search_elt (&self) -> Vec<String> {
        match match &self.command {
            Some(x) => x,
            None => {panic!("Aucune commande est saisie")},
        } {
            Action::Scan{path:_} => None.expect("Ne dispose pas d'élement"),
            Action::Search{path:_, rest} => {rest.to_vec()},
            Action::Write2md { request:_, path_out:_, path_in:_, rest } => rest.to_vec(),
            Action::Writeplaylist { request:_, path_out:_, path_in:_, rest } => rest.to_vec(),
            Action::Writeplaylistjson { path_out : _ } => panic!("Write depuis un JSON ne nécessite pas un filtre."),
            Action::Tag { elt } => elt.to_vec(),
            Action::Scrap { path:_ } => panic!("Scrap ne nécessite pas un filtre."),

        }
    }

    /// Renvoi le chemin pour créer le fichier md ou la playlist. 
    pub fn md_path_out(&self) -> &std::path::Path {
        match match &self.command {
            Some(x) => x,
            None => {panic!("Aucune commande est saisie")},
        } {
            Action::Scan{path:_} => None.expect("Ne dispose pas d'élement"),
            Action::Search{path:_, rest:_} => None.expect("Ne dispose pas d'élement"),
            Action::Write2md { request:_, path_out, path_in:_, rest:_ } => path_out,
            Action::Writeplaylist { request:_, path_out, path_in:_, rest:_ } => path_out,
            Action::Writeplaylistjson { path_out } => path_out,
            Action::Tag { elt:_ } => panic!( "Pas de path_out pour Tag"),
            Action::Scrap { path:_ } => panic!("Scrap ne dispose pas de path_out."),
        }
    }
}
