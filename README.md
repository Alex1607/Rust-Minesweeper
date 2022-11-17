# Rust Minesweeper
This repository was made as an experimental project I wanted to write in Rust.  
It should be able to generate Minesweeper fields in any size wanted, fill it with mines and solve it.

## How the field is generated
There will be several ways of generating as field:
- Default
- Modified
- No Guessing

Where default is the de facto standard generation use by most minesweeper generators. It will make sure the first click is not a mine by moving the mine when the first click is on a mine.  
Then there is the Modified generator, it will generate a field that guarantees a 0 field at the start. No matter where the field is.  
And last but not least: The No 50/50 generator. This one generates a field which is solvable without any guessing at all.

However, since it's quite hard if not even impossible to generate these fields before the user clicked on a field, especially the no guessing field, the generator will generate the field only after the user clicked on a field.

For the no guessing generator, the generator will generate a modified field and then try to solve it. If it's able to solve it from the same position as the user clicked on, it's deemed solvable. If not, it will generate a new field and try again. Effectively brute forcing a field.

## How does the solver work
In order to solve around 90% of a minesweeper fields, it's enough to just do a simple algorithm:
- Find all fields that are open and have a value above zero
- For each found field, count all fields that are flagged around it
- Check if the value of the field + flags around it, is the same value as the closed fields around the field
    - If it's the same, all fields around can be flagged
- After that again loop over all fields with an above zero value
- Check if the value of the field is the same as the flags placed around it
    - If it's the same, all fields around can be opened

But, most of the time, this is not enough for the endgame. For this, there is the tank solver. It's a slow and heavyweight backtrack solver which is able to solve any conceivable position.  
The algorithm was designed by https://luckytoilet.wordpress.com/2012/12/23/2125/. This site also contains a wonderful visualization of the algorithm.



**Currently WIP**