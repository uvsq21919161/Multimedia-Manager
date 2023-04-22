# Multimedia-Manager

Projet effectué dans le cadre de mes études.  

L'objectif de ce mini-projet est de créer une application en ligne commande pour gérer une collection de fichiers multimédia.  
L'application proposera une double interface utilisateur :
* les arguments pourront être passés en paramètre de l'exécutable,
* si l'exécutable est lancé sans argument, un mode interactif sera proposé à l'utilisateur.

L'utilisateur saisira une commande suivie d'arguments.
Les commandes à implémenter sont les suivantes :

* `scan` : analyser récursivement un répertoire pour collecter les fichiers supportés
** l'analyse doit extraire les métadonnées du fichier
* `search` : effectue une recherche sur les données gérées (format de la requête ?)
* `scrap` : récupérer des données sur le web pour un ensemble de fichiers
* `write2md` : génèrer un fichier _Markdown_ contenant le résultat d'une requête
* `writeplaylist` : générer des playlists, ...
* `tag` : ajoute un tag à un ensemble de fichiers (requêtes).
Les données analysées peuvent être sauvegardées au format JSON pour une réutilisation ultérieure.

---
# Exécution

Pour exécuter le code, placez-vous d'abord dans le chemin correspondant dans le terminal, puis exécutez la commande.
```
cargo run <cmd>
```
<cmd> correspond à l'une des commandes décrites précédemment.
