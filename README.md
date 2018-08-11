# ytesrev

<img src="ytesrev.mp4" alt="showcase" width="200px"/>

This is a presentation tool inspired by 3blue1brown's tool [manim](https://github.com/3b1b/manim). Currently it
supports rendering LaTeX files, relatively simple layout, reading PNGs among other things.


## Differences to manim

The biggest difference is the goal of the two programs. Manim is made to render a video, while ytesrev is made to work
in live situations where reactivity and dynamicity are valuable. For example, in manim, screen size and timing are
pretty much constant, where ytesrev has to have a dynamic layout engine and an event system.


## Usage

An example of a presentation with ytesrev can be found in the `src/main.rs` file. ytesrev will become more of a library
in the future, but currently to use this you have to clone the repository and modify the source. This will be changed in the
future.


## Project structure

* `src/window/`: Contains the `WindowManager` which is responsible for creating the window, managing events and timings and keeping track of the slides
* `src/scene/`: Abstract definitions of a scene (slide) and a wrapper for `Drawable`s
* `src/drawable/` Abstract definitions of drawable objects as well as drawing positions
* `src/layout/`: Definitions and implementations of layouts (stacking and splitting)
* `src/image/`: Loading PNGs
* `src/latex/`: Rendering LaTeX expressions
* `src/ditherer/`: To create those cool text 'whoosh' effects
