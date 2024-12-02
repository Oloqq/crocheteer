# 1. Introduction

Crocheting is a manual craft that involves weaving a yarn stitch by stitch to create a piece of fabric. The craft is commonly confused with knitting, although the techniques are vastly different.

![alt text](images/image-4.png)
https://www.thesprucecrafts.com/differences-between-knitting-and-crochet-4077447

Crocheting makes it easy to create 3-dimensional shapes, and the pieces are generally sturdier than knitted ones, making the technique well suited for things that should keep their shape such as decorations or toys.

> crochet as a tool to visualize hyperbolic geometry
> https://www.cabinetmagazine.org/issues/16/wertheim_henderson_taimina.php

This project focuses on using crocheting to create 3-dimentional, stuffed shapes, an art called [amigurumi](https://en.wikipedia.org/wiki/Amigurumi).

The person crocheting, works by following a *pattern* that describes the sequence of stitches necessary to achieve a certain shape. Designing such a pattern is trivial for axially symmetrical shapes, but otherwise requires much trial and error.

>dino's body+tail is an example of non-symmetrical shape\
>ai generated, but proves the point\
>sites providing patterns use ai images to lure artists by using more attractive shapes, that they don't have patterns for \
>![alt text](images/image-5.png)
> https://theamigurumi.com/wp-content/uploads/2024/01/Crochet-Dinosaur-Trex-Amigurumi.jpg
>

There are currently no tools that could help with designing such shapes. The closest thing is [Crochet Lathe](https://avtanski.net/projects/crochet/lathe/), however the tool is only applicable to simple, axially symmetrical shapes.

## 1.1 Objectives

The goal of this project is to create a tool, that would let an artist visualize the plushie that their pattern produces before commiting hours into actually crocheting it.

The idea here is to build a graph representing the fabric, and then treat it as a [force directed graph](https://en.wikipedia.org/wiki/Force-directed_graph_drawing) ([see a live demo](https://vasturiano.github.io/force-graph/example/directional-links-particles/)), with forces specifically designed so that as forces diminish, the spatial graph looks like the crocheted piece that would be created from the same pattern.


## 1.2 Overview
Section 2 describes how a crochet piece is built. It describes the techniques referenced later in the work, and the considerations necessary to handle them in proposed algorithms.

Section 3 introduces ACL (Amigurumi Crochet Language), the formal language used to describe crochet patterns.

Sections 4, 5 and 6 describe how the ACL pattern is processed into a 3D visualization.

![alt text](image.png)
Figure shows a high level view of the process.

Section 7 showcases a GUI used while developing these algorithms. It also describes the interfaces used for communication between the calculation (backend) and display (frontend) components.

Section 8 summarizes the work related to the project.