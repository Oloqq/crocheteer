# Problem

- Crocheting multiple parts, then joining them together is not considered at this point.
- Some inspirations for the grammar are in [examples folder](./examples%20of%20patterns/).

- For genetic programming the pattern may be
  - single vector of stitches
    - round numbers determined automatically
  - a vector of rounds (that is a vector of vector of stitches)
    - how to assure stitch numbers match?

- Common abbreviations
  - sc: single crochet
  - inc: increase
  - dec: decrease / invisible decrease
  - R: round number
  - MR: magic ring
  - FO: fasten off (finish)
    - automatically at the end
  - front/back loops
    - implementing these will require a different approach in the `Plushie` (working on angles between vectors or smth)
    - FLO: Front Loop Only
    - BLO: Back Loop Only
  - slst: slip stitch
  - skip

- Common grammar
  - each line
    - is a round
    - starts with `RX:`, where X is the number
      - sometimes rounds are repeated e.g. `R2-R4:`
    - ends with `(X)` where X is the number of anchors produced
  - `[A B] x X` denotes `A B` repeated `X` times
    - there are many conventions with differing brackets

# Grammar (human readable)
starting rule: **pattern**

```js
pattern -> starter `\n` round* `FO`?

starter -> magic_ring // might add support for starting from a chain in the future

magic_ring -> `R` `1` `MR` X

round -> round_id `:` stitches count `\n`

round_id ->
    `R` X |
    `R` X `-` `R` X

count -> `(` X `)`

stitches -> repeated | action (`,` action)*

repeated -> `[` stitches `]` `x` X

// another actions may be: transferring to another stitch, reversing order etc.
action -> X? STITCH

STITCH -> // case insensitive
    sc |
    dec |
    inc |
    skip // add more stitches when supported


X -> `[1-9][0-9]*
```

# Grammar (genetic)

```js
pattern -> MR X (X stitch)*

stitch ->
    SC |
    DEC |
    INC |
    SKIP

X -> `[1-9][0-9]*
```