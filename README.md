# Living Neural
Living Neural implements the simplest form of a neural automata and renders the output using OpenGL.

## Examples


https://user-images.githubusercontent.com/64916254/188502278-0e4fa828-2ec1-4c89-848c-4a75c0cd84a3.mp4


https://user-images.githubusercontent.com/64916254/188502305-d7a4bf24-de5a-4858-8a8a-14baaa49a0d6.mp4



https://user-images.githubusercontent.com/64916254/188502320-76ff9169-94df-4325-aaaa-5cd904c3edcd.mp4



## Usage
```bash
Living Neural 0.1.0
Tooraj Taraz <tooraj.info@gmail.com>
Living Neural implements a simple neural automata accelerated by OPENGL.

USAGE:
    living_neural [OPTIONS]

OPTIONS:
    -a, --activation <activation name or source>                                                           Sets the activation function.
    -c, --color <desired color name>                                                                       Sets the fg color. All the valid CSS3 colors are acceptable.
    -C, --COLOR <desired color values. [R, G, B] => '[0.2, 0., 1.0]' each must be between 0.0 and 1.0.>    Sets the fg color.
    -f, --fps <desired frame count per second>                                                             Sets the FPS. Must be 32 bit positive floating point and greater than 0.0.
    -h, --height <desired window height>                                                                   Sets initial window height, can be resized later. Must be 32 bit unsigned int and greater than 0.
        --help                                                                                             Print help information
    -k, --kernel <desired kernel name/mode>                                                                Sets the kernel. It can be random, worm, fiber, waves, rule30, gameoflife or custom.
    -K, --ckernel <desired kernel array>                                                                   Value for custom kernel. It must be an array of length 9 and wrapped in qoutation marks. Example: [1.0, 2.3, 0., 0.0, 0.0, 0.0, -1.23421, 8.0, 1.0]
    -p, --persistent                                                                                       If passed extends the fragment shader with pixel persistence.
    -s, --skip <frames to skip>                                                                            Sets the number of frames to skip.
    -V, --version                                                                                          Print version information
    -w, --width <desired window width>                                                                     Sets initial window width, can be resized later. Must be 32 bit unsigned int and greater than 0.
```
