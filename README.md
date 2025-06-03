# Bevyjam6

This project was generated using the [Bevy New 2D](https://github.com/TheBevyFlock/bevy_new_2d) template.
Check out the [documentation](https://github.com/TheBevyFlock/bevy_new_2d/blob/main/README.md) to get started!

# Components

## Bipper

- Un mesh qui a `bipper:n` dans le nom, où n est l'ID du fichier sonore `bipper{n}.ogg`
- Il faut un fichier son avec l'ID existant (exemple: bipper4.ogg)

## Blink

- Requires Synchronized, sinon par défaut ce sera la track 0 
- Un mesh qui a `blink:n` dans le nom, où n est 0, 1 ou 2 sera synchronisé avec la musique et clignotera (alterne entre emissive on et emissive off). 
- Il faut que le material soit emissive dans l'onglet render de magicavoxel

## Bumper

- Un mesh qui a `bumper:x:y` dans le nom, où x et y sont des float correspondant respectivement au scale hover/click
- Il est possible de rajouter `bumper` dans le nom sans paramètres (afin d'utiliser des paramètres par défauts)

## Slider

- TODO

## Synchronized

- Peut suivre la track 0, 1 ou 2 pour s'adapter à différents beats de la musique

## Musique

-Track 1 90bpm
-Bip 1 /8
-Bip 2 *1.5
-bip 3 1


-Track2 100 bpm (0.6s)

bip1 : /2 (1.2s)
bip2 : x1 (0.6s)
bip3 : x1 (0.6s)
bip4 : /8 (4.8s)


bip5 : /2 (1.2s)
bip6 : x1 (0.6s)
bip7 : x1 (0.6s)
bip8 : /8 (4.8s)

