# How to develop:

To ease development a little, there's a couple of useful tools.

Cargo watch rebuilds project each time some change in sources occured:

    $: cd shipico
    $: cargo watch -x build

Basic http server will serve wasm files to your browser:

    $: cd shipico
    $: basic-http-server .

To install them run:

    $: cargo install watch
    $: cargo install basic-http-server

After you will launch them (probably in different terminals) you can visit `localhost:4040/deploy/index.html` to
face results of your work.

Each time you will change sources, wait half a second and reload window for changes to take effect.

# TODO:

## Node tree basics:

- [ ] Make proper scale from mouse pointer
- [ ] Line shader or Curve shader for connectors (or maybe just draw lines from textured rects)
- [ ] Spritesheet code (store packed textures and stuff)
- [ ] Sprite (rect with a texture)
- [ ] Sprite with modular pieces (corner, side and center)
- [ ] Draw text (check `not-fl3` github user, he got miniquad compatible font package somewhere)

<!---
In case you wandering why textures.
Ask yourself a question: we can draw lines, but can we fill them?
If answer is yes, then i got another one for you:
You want to write a ton of shader code to make it happen?

I am not.
-->

## Node tree textures:

- [ ] Circle empty
- [ ] Circle filled (or maybe a better way to draw circles? I vote for shader, buuut it's more work than plain texture)
- [ ] Node background
- [ ] Node side
- [ ] Node corner
- [ ] White primitive rect

<!---
More textures to be added. Push them to ./assets/textures/...
-->

## Meshes:

- [ ] Rect (really we could just insert rect as code)

## Node tree UI:

- [ ] Composed Node (with inputs and stuff)
- [ ] TBA

## Other:

- [ ] PR to miniquad with Mat3 support
