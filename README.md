# ytesrev

<img src="ytesrev.gif" alt="showcase" width="400px"/>
<sub><sup>
    (higher quality in real life, gif looks like trash thanks to compression and me not knowing how to make it better)
</sup></sub>

This is a presentation tool inspired by 3blue1brown's tool [manim](https://github.com/3b1b/manim). Currently it
supports rendering LaTeX files, relatively simple layout, reading PNGs among other things.


## Differences to manim

The biggest difference is the goal of the two programs. Manim is made to render a video, while ytesrev is made to work
in live situations where reactivity and dynamicity are valuable. For example, in manim, screen size and timing are
pretty much constant, where ytesrev has to have a dynamic layout engine and an event system.


## Usage

Check out the `src/example/` folder.


## Project structure

* `src/anchor/`: To keep things anchored to one side of the screen
* `src/ditherer/`: To create those cool text 'whoosh' effects
* `src/drawable/` Abstract definitions of drawable objects as well as drawing positions
* `src/empty/`: The empty object
* `src/image/`: Loading PNGs
* `src/latex/`: Rendering LaTeX expressions
* `src/layout/`: Definitions and implementations of layouts (stacking and splitting)
* `src/margin/`: To give some object a margin
* `src/scene/`: Abstract definitions of a scene (slide) and a wrapper for `Drawable`s
* `src/solid/`: A rectangle of a solid color
* `src/window/`: Contains the `WindowManager` which is responsible for creating the window, managing events and timings and keeping track of the slides
* `src/withsize/`: Give an object a constant size
