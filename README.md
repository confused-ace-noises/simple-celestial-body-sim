# Celestial body simulator

A very simple celestial body simulator, made in bevy, aimed to be a starting point of a more complex simulator.

## Example 
To launch the simulator, you must first define a JSON file that describes every aspect of each body you want to simulate, like this:
```json
[
    {
        "position": [0.0, 0.0, 0.0],
        "radius": 5.0,
        "color": [255, 0, 0, 255],
        "light": true,
        "velocity": [0.0, 0.0, 0.0],
        "acceleration": [0.0, 0.0, 0.0],
        "mass": 10000000000.0
    },
    {
        "position": [8.0, 8.0, 1.0],
        "radius": 1.0,
        "color": [0, 255, 0, 255],
        "light": false,
        "velocity": [0.0, 0.0, 0.0],
        "acceleration": [0.5, 0.0, 0.0],
        "mass": 100.0
    }
]
```
Here is the use for every field:
- **position**: the position of the body in 3d space.
- **radius**: the radius of body.
- **color**: must be given in RGBA format, useful to identify the bodies one from the other.
- **light**: whether the body is a light emitter, such as a star.
- **velocity**: velocity of the body, as a vector in 3d space.
- **acceleration**: the acceleration of the body as a vector in 3d space.
- **mass**: the mass of the body.

Every field that accepts a 3 element array describes a vector or position in 3d space with the x axis as the 1st element, the y axis (vertical) as the 2nd, and the z axis as the 3rd element.

Once you've set that data, run:
```
$ celestial-body-sim data.json
```

As long as space isn't pressed, movement of the bodies will stay still. Once space is pressed (and held down) the simulation of velocity, acceleration and gravity will resume.

## The simulator in action

gif_here

this orbit was achieved with the following JSON config:

```json
[
    {
        "position": [0.0, 0.0, 0.0],
        "radius": 1.0,
        "color": [255, 0, 0, 255],
        "light": true,
        "velocity": [0.0, 0.0, 0.0],
        "acceleration": [0.0, 0.0, 0.0],
        "mass": 9.58001e12
    },
    {
        "position": [8.0, 8.0, 1.0],
        "radius": 0.5,
        "color": [0, 255, 0, 255],
        "light": false,
        "velocity": [2.0, -3.6, 0.0],
        "acceleration": [0.0, 0.0, 0.0],
        "mass": 600000.0
    }
]
```