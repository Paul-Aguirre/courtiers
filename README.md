## Description
### What this is
This project aims at replicating the logic of the game *Courtisans*.
It is intended as an exercise to perfect my Rust skills.

### The game
*Courtisans*, translated to *Courtiers* in English, is a French card game about influencing the status of six families.
Members of a family that is *in the light* will score points to whoever has them on their side while members of a *disgraced* family will deduct points.
The status of families changes as members are placed at the *Queen's table*.
Most family members have special effects that further impact the domain they are placed in.
Final objective being to score more point that the other players.

## Roadmap
### Modifying the domain's modelling
#### To do

#### Ongoing

#### Done
- Using the the new `Piles` enum in player.rs and queens_table.rs to implement the Typestate pattern and make invalid states impossible to represent
- Implement the Typestate pattern with `Player` and `QueensTable` instances

### Implementing a Client - Server system
#### To do
- The server will maintain the clients' state and spies piles on which it only communicates for rendering
- Internalized - Externalized layer (as a protection against unsanitary inputs): deserialization / validation
(- ? enum et enum subset)

#### Ongoing

#### Done
