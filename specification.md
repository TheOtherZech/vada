# Vada: Zech's Nightmare of a Data Language

**TL;DR:** Cuelang + SQL with YAML-like syntax

Vada is a data language designed to take the pain out of composing, validating, and transforming build artifacts. It's a language that handles like a database, featuring declarative constructs for data schemas, field constraints, relational constraints, manifests, and dynamic accessors. It's easy to write, easy to read, and easy to reason about.

More specifically, Vada is a data language designed for composing, validating, and transforming the kinds of *aggregate data* artifacts commonly seen in game development. It's built for wrangling large collections of entity data, translation strings, asset descriptors, and package maps, spread across multiple files in multiple directories. 

Consequently, this means that Vada is not just a language spec, but also an AST spec and an evaluator spec. Most of Vada's utility comes from the tools that can be built on top of it; the evaluator is built to be hacked on, allowing for the easy extraction of both lowered and un-lowered trees for easy serialization and alternative constraint evaluation.

Vada takes inspiration from CUE, python, and (of course) JSON. In the places where Vada _does_ diverge from these common touch-points, it does so in ways that are eminently readable and understandable. It uses consistent symbolic prefixes for core record types, predictable digraphs for mathematical operators, and easily readable constraint expressions. 

While you can do some pretty complicated stuff with it, one of Vada's core principles is that it's a language that's easy to reason about.

## But where's the spec?

The spec isn't concrete yet. Go check the official website (over at vadalang.org) for the current overview.