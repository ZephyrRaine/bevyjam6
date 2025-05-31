# Bevyjam6

This project was generated using the [Bevy New 2D](https://github.com/TheBevyFlock/bevy_new_2d) template.
Check out the [documentation](https://github.com/TheBevyFlock/bevy_new_2d/blob/main/README.md) to get started!

# Components

## Blink

- Requires Synchronized, sinon par défaut ce sera la track 0 
- Un mesh qui a `blink n` dans le nom, où n est 0, 1 ou 2 sera synchronisé avec la musique et clignotera (alterne entre emissive on et emissive off). 
- Il faut que le material soit emissive dans l'onglet render de magicavoxel  

## Synchronized

- Peut suivre la track 0, 1 ou 2 pour s'adapter à différents beats de la musique

## Musique

-Track 1 90bpm
-Bip 1 /8
-Bip 2 *1.5
-bip 3 1