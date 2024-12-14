# 1. Introduction

Crocheting is a manual craft that involves weaving a yarn stitch by stitch to create a piece of fabric. The craft is commonly confused with knitting, although the techniques are vastly different.

https://www.thesprucecrafts.com/differences-between-knitting-and-crochet-4077447

Crocheting makes it easy to create 3-dimensional shapes, and the pieces are generally sturdier than knitted ones, making the technique well suited for things that should keep their shape such as decorations or toys.

> crochet as a tool to visualize hyperbolic geometry
> https://www.cabinetmagazine.org/issues/16/wertheim_henderson_taimina.php

This project focuses on using crocheting to create 3-dimentional, stuffed shapes, an art called [amigurumi](https://en.wikipedia.org/wiki/Amigurumi).

The person crocheting, works by following a *pattern* that describes the sequence of stitches necessary to achieve a certain shape. Designing such a pattern is trivial for axially symmetrical shapes, but otherwise often requires trial and error.

There are currently no tools that could help with designing such shapes. The closest thing is [Crochet Lathe](https://avtanski.net/projects/crochet/lathe/), however the tool is only applicable to simple, axially symmetrical shapes.

## 1.1 Objectives

The goal of this project is to create a tool, that would let an artist visualize the plushie that their pattern produces before commiting hours into actually crocheting it.

The idea here is to build a graph representing the fabric, and then treat it as a [force directed graph](https://en.wikipedia.org/wiki/Force-directed_graph_drawing) ([see a live demo](https://vasturiano.github.io/force-graph/example/directional-links-particles/)), with forces specifically designed so that as forces diminish, the spatial graph looks like the crocheted piece that would be created from the same pattern.

## 1.2 Proposed solution
A domain specific language, named *ACL*, shall be introduced, designed for expressing amigurumi patterns (section 3). ACL's parser shall transform the pattern into a list of stitches and other actions performed in order. Another algorithm shall then transform that list into a graph that represents how stitches are connected with one another (section 4). As this is in some way simulating the actions performed with a crochet hook, we call that algorithm *Hook*.

Nodes of the graph shall be assigned positions in 3D space, and forces shall act on the graph so those positions and the overall shape of the graph become aligned with what a real crocheted piece would look like (section 5). In this paper, we call that force-directed graph a *Plushie*.

Two forces acting on a Plushie are necessary. First one, *link force*, shall try to keep connected stitches at a specific distance from one another. Second one, *stuffing force*, shall simulate the effect of stuffing inside the piece on the fabric. The first force is trivial, but for the second one, we need to get a notion on where is the inside of the Plushie. Two methods of applying a force from the inside are explored in section 5, one that is quick but occasionaly fails on complex shapes, and another one that involves proper skeletonization using algorithm introduced in [[2]]

![alt text](image.png)
Figure shows a high level view of the process.

All algorithms described in this paper are implemented in project *Crocheteer*, available as open source on github[[1]]. Crocheteer is split into backend and frontend. The algorithms described in the paper are part of the backend, written in Rust for performance and correctness. Web frontend, developed with TypeScript and webpack, handles the display. Either part can be swapped for another piece of software as long as it conforms to the defined API. Crocheteer's web frontend is described in detail in section 6.

[1]: https://github.com/Oloqq/crocheteer
[2]: https://arxiv.org/pdf/1912.11932