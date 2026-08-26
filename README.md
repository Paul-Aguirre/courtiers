## Roadmap
### Modifs logique de domaine
#### Ongoing
#### Done
- utiliser la nouvelle enum Piles dans player.rs et queens_table.rs pour mettre en oeuvre le Typestate pattern et rendre impossible la représentation d'états invalides
- mettre en place le TypeState pattern avec les instance de Player et QueensTable ??
### modèle client serveur
- le serveur maintient l'état du client plus les piles des espoins sur lesquels il ne communique qu'à la fin pour du rendu graphique
- couche internalised - extenalied (protection contre les inputs non sanitized):  deserialisation / validation
(- ? enum et enum subset)
