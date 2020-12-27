# IdPaths

In `z2edit`, an *IdPath* is a resource identifier for an internal `z2edit`
data structure, and often, its corresponding data in the Zelda2 ROM.

*IdPaths* are meant to provide a hierarchical representation of the
information that would go into making a ROM hack.  Typically, *IdPaths*
have multiple components (although this is not univerally true),
representing specific editable features or selections.

### Examples

Here are some example *IdPaths* representing some game heirarchy:

Overworld:

- `west_hyrule`: The Western continent overworld map.
- `west_hyrule/33`: Sideview area 33 on the West contientent (aka Trophy Cave).
- `west_hyrule/enemy/11`: A jumping red Octorok.
- `west_hyrule/connector/1`: Overworld connector 1 in West Hyrule
   (connects overworld to Trophy Cave).

Town:

- `town/4`: The second sideview area of Ruto (where Jump Spell House is
   located).
- `town/enemy/16`: The Red Lady guarding Jump Spell House.
- `west_hyrule/town/14`: The text `GORIYA OF|TANTARI|STOLE OUR|TROPHY.`.
- `west_hyrule/connector/47`: Overworld connector 47 in West Hyrule
  (connects to Ruto).

Palace:

- `palace_125/3`: The Palace 1 room with the first key.
- `palace_125/enemy/1f`: A Red Stalfos.
