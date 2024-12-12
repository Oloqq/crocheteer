# Formal language for amigurumi patterns

<!-- https://www.montana.edu/extension/blaine/4-h/4h_documents/CrochetMadeEasy.pdf -->

To process a crochet pattern, we need to express it using a formal language.
 <!-- Maybe we don't and
 Parsing Semi-structured Languages: A Crochet Pattern to Diagram Translation, https://www.springerprofessional.de/en/parsing-semi-structured-languages-a-crochet-pattern-to-diagram-t/25864374
 could show that, but I gotta find non-paywall version -->

Patterns shared in the community have equivalent overall structure, but do not follow a strict grammar. For example, repetition of stitch *Sc* 7 times could be seen written as "7 SC", "[SC] x 7", "(sc) 7 times" and more.
<!-- if this needs a source, it would have to be random patterns online
https://yarnsociety.com/patterns/celia-the-crab/
https://www.themaryjay.com/free-patterns/crochet-turtle-pattern
https://www.tinycurl.co/how-to-amigurumi-crochet/#stitches -->

This work shall define a domain specific language, *Amigurumi Crochet Language (ACL)*, that should be easily understood by creators that can read a pattern found online.

## Amigurumi Crochet Language grammar
### Starter
An amigurumi piece begins with either MR (Magic Ring) or Ch (Chain), so an ACL document also begins with one of them. MR or Ch can be constructed from an arbitrary number of stitches, so those instructions take an argument specifying that number.

```
MR(6)
```
```
Ch(18)
```

The only instructions allowed before a starter are *metadata* and yarn describing *operations* (defined in further subsections).

### Rounds
After the starter, an amigurumi piece is worked stitch by stitch in a spiral. A set of stitches going all around the current piece is called a *round*.
A round is denoted by its number, then a colon (`:`), followed by a list of stitches (details below).
```
R1: 6 sc
```

Crochet patterns typically display how many stitches the user should have available after each round. In ACL, that part is optional, and denoted as an integer in parentheses after the stitches.
```
R1: 6 sc (6)
R2: 6 inc (12)
```

Sometimes a whole round needs to be repeated several times, which is denoted with a range of rounds.
```
R1-R2: 6sc
```

While a user is developing their pattern, it would be a nuisance to keep correcting the changing round numbers. Therefore, round numbering in ACL is optional, and round repetition can be set by just putting an integer before the colon.
```
: 6 sc
2: 6 sc
```

### Stitches
ACL currently works with 3 types of stitches:
- sc - [single crochet](https://www.thesprucecrafts.com/single-crochet-stitch-tutorial-979083)
- inc - [increase](https://thewoobles.com/pages/how-to-single-crochet-increase)
- dec - [decrease](https://www.thesprucecrafts.com/how-to-decrease-in-crochet-4002866)
<!-- TODO find a ONE book/blog that describes all the stitches, it's silly to link different websites for different stitches -->

Stitches are only placed inside *rounds*. Each round contains a comma-separated list of stitches. Each stitch is optionally preceded by a number stating how many times it should be repeated.
```
sc, inc, 2 sc, inc, sc
```

Sometimes parts of a round are repeatable. These parts can be described using ACL's subpatterns
```
[sc, inc] x 2, 2 sc
```
is equivalent to
```
sc, inc, sc, inc, sc, sc
```

### Operations
<!-- TODO action would be a better name, refactor the code -->
The person crocheting has a few actions available besides creating new stitches, in the context of ACL we call them operations. With a few exceptions, operations are permitted either between stitches of a round or between rounds.

#### Fasten-off
When the piece is done, the last round is typically squeezed together by performing a [fasten-off (FO)](https://crochettoplay.com/how-to-fasten-off-in-amigurumi/). `FO` is only allowed between rounds. No stitches may be placed after FO, unless `goto` operation is used.

```
...
: 6 dec
FO
```

#### Back-loop-only, front-loop only, both-loops
Each existing stitch can be split into 2 loops. One on the inner side (back-loop), one on the outer side (front-loop). By default, new stitches are anchored to both of those loops. Using only one loop creates a sharper corner in the piece.
Using operations `BLO` and `FLO`, the default behavior can be altered so that new stitches are anchored to just the back-loop or just the front-loop. Default behavior is restored at the start of next round or by using `BL`, which stands for both-loops.

```
: FLO, 12 inc
: 24 sc
: 6 sc, BLO, 12 sc, BL, 6 sc
```

#### Mark and goto
Operations `mark` and `goto` are used in pairs to mark a working position, and return there later. Both operations require an identifier for the position.
For example, following code closes a piece by working on the back-loop, fastens-off, and then returns to continue working from the front loop.

```
...
: 6 sc
mark(point_of_split)
: BLO, 6 sc
FO

goto(point_of_split)
: FLO, 6 inc
: 12 sc
...
```

#### Yarn color
`color` operations allows setting color for part of the piece in the visualization. Takes 3 integer arguments, representing the RGB components of the color.

```
color(255, 255, 0)
: 6 sc, color(0, 0, 255), 12 sc
```

### Comments
Comments start with a hash `#` and continue until the end of the line. There are no multi-line comments in ACL.

```
color(255, 0, 0) # switch to red yarn
```

### Metadata
<!-- TODO this doesnt really fit the definition of metadata, refactor the code so its called parameters or smth -->
Patterns may need a specific simulation/visualization setup. ACL grammar handles that by accepting arbitrary key-value pairs using the following syntax.
```
@key = value
@centroids = 2
```

## Sample ACL pattern
The following pattern describes a simple mushroom shape.
```
@centroids = 2

color(160, 80, 40)

MR(6)
R1: 6 inc (12)
R2: 12 sc
R3-R4: 12 sc (12)
mark(cap_start)
R5: BLO, 6 dec (6)
FO

goto(cap_start), color(255, 255, 0)
R6: FLO, 12 inc (24)
R7-R8: 24 sc (24)
R9: [dec, sc] x 8 (18)
R10: [dec, sc] x 6 (12)
R11: 6 dec (6)
FO
```

## Implementation
ACL parser and lexer are generated using [pest](https://pest.rs). Pest generates these based on a [parsing expression grammar (PEG)](https://en.wikipedia.org/wiki/Parsing_expression_grammar) defined using a [custom syntax](https://pest.rs/book/). The following code defines ACL using that syntax.
```
program = { SOI ~ (round | comment | meta | control | NEWLINE)+ ~ EOI}

meta    = { "@" ~ IDENT ~ "=" ~ IDENT ~ LINEEND }

round       =  { roundspec? ~ ":" ~ stitches ~ round_end? ~ LINEEND }
roundspec   = {
    round_range
  | round_index
  | NUMBER
}
round_index = @{ R ~ NUMBER }
round_range = @{ R ~ NUMBER ~ "-" ~ R ~ NUMBER }
round_end   =  { "(" ~ NUMBER ~ ")" }

stitches   = { stitch_sequence ~ ("," ~ stitch_sequence)* }
stitch_sequence  = {
      (NUMBER ~ KW_STITCH)
    | KW_STITCH
    | interstitchable_operation
    | repetition }
repetition = { "[" ~ stitches ~ "]" ~ "x" ~ NUMBER }

control = { operation_sequence ~ LINEEND }
operation_sequence = { operation ~ ("," ~ operation)* }
arg_int   =  { "(" ~ NUMBER ~ ")" }
arg_int_3 = _{ "(" ~ NUMBER ~ "," ~ NUMBER ~ "," ~ NUMBER ~ ")" }
arg_ident =  { "(" ~ IDENT ~ ")" }


operation = {
      interstitchable_operation
    | (KW_MR ~ arg_int)
    |  KW_FO
}
interstitchable_operation = {
      KW_BLO
    | KW_FLO
    | KW_BL
    | (KW_MARK ~ arg_ident)
    | (KW_GOTO ~ arg_ident)
    | (KW_COLOR ~ arg_int_3)
    | (KW_CH ~ arg_int)
}

KW_MR       = { "MR" }
KW_FO       = { "FO" }
KW_MARK     = { "mark" }
KW_GOTO     = { "goto" }
KW_FLO      = { "FLO" }
KW_BLO      = { "BLO" }
KW_BL       = { "BL" }
KW_CH       = { "ch" | "Ch" }
KW_COLOR      = { "color" }
KW_STITCH   =  { "sc" | "inc" | "dec" }

comment     = _{ "#" ~ not_newline* ~ (NEWLINE | EOI) }
not_newline = _{
    !(NEWLINE) ~ ANY
}

LINEEND     =  { NEWLINE | comment | EOI }
PLACEHOLDER =  { "_" }
R           =  { "R" }
ALPHA       =  { 'a'..'z' | 'A'..'Z' }
IDENT       = @{ (ALPHA | DIGIT | "_")+ }
NUMBER      = @{ (NONZERO ~ DIGIT*) | "0" }
NONZERO     = _{ '1'..'9' }
DIGIT       = _{ '0'..'9' }
WHITESPACE  = _{ " " }
```

The PEG above would be equivalent to the following BNF grammar.
<!-- TODO: BNF notation -->
```
```