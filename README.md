# Dig Site 8
octo jamming in 2018

and 2019

## Colors
- Background: `#163a5c`
- Foreground 1: `#212946`
- Foreground 2: `#6186a9`
- Blended: `#921b30`
- Buzzer: `#252a2f`
- Silence: `#252a2f`


## Game Structure
The main structure is a game loop. The game loop performs system processing (audio and input)
once per frame. All other operations are performed by gamestate specific code. Each state has
a function that is called once per frame, and a function that is called repeatedly until
the frametime is over.

## Game States
The code for each state is in a file in `src/states`. Each state is given a number in
`src/states/state_header.o8`. A table that has pointers to the state code (the order must match the header) is
defined in `src/states/state_data.o8`.

## Input
Input is handled in `src/input/input.o8`. Currently, it debounces the input.
`src/input/input_data.o8` is the table of keys per state. Is this a good way to do
it? I think it minimizes code space.

## Audio
Music plays in the background, which means we have 2 frames before we need to be
filling the buffer again.

Right now SFX blocks, but I don't see why it would have to.

`src/audio/sample_header.o8` is for audio related constants (possible to remove it?)
`src/audio/sample_data.o8` is the raw data for the sound effects.


## Text
Right now, text is a bit of a mess. The text generator should know about the font
so it can embed pixel widths with words. It works but it is walking through each
word to figure out the length every time it draws it!

## Screen Drawing
The code in `src/ui/screen.o8` is a system to display predefined screens that
consist of text and sprites.




## Assets
`assets/` contains all of the game assets. These are all 'raw' in that they have
not been processed for use in the game. For example, images need to be turned
into byte data before they can be used.

The directory structure is below. I think some of these could stand to be renamed
or reorganized.

- Enemies: Images of enemies
- Fonts: unused?
- Images: Large images (not tiles or portraits)
- Portraits: There are 2 heros in here
- Prefabs: These are `toml`s with data about biomes, enemies, events, treasure, and weapons
- Sound: Sound Effects
- Terrain: Terrain tiles
- Tiles: One floor tile

## Other Things That Don't Have a Section Yet
- Graphics
- Status effects
- Map Rendering
- Map Generation
- Enemy pathfinding, AI
- Inventory
- Weapons and upgrades
- Skills and abilities
- Victory condition
