
# Mango

Mango is designed to be a quick & easy tool for processing data use node networks. The idea is to
provide common operations in a more powerful, flexible & discoverable way than the commandline.
Simple data typing along with easy to find, useful transformations make it an ideal tool for quickly
messing around with data.


## Status

The project is in early alpha. It is fun to play around with as a concept but there is lots of work
still to do.

This is also my first Rust project of any size so that code is likely to be horrible and heavily
influenced by my time spend with C++ & Javascript.


## Motivation

As a web developer, I spend more time that I would like, writing little scripts to transfer json
data and other text. Rather than writing a bespoke script each time with the effort needed to get
the syntax & flow correct, I would like to plug some nodes together in a responsive editor and have
them do the processing for me.


## Use cases

There are two use cases:

- Allow users to pipe data into Mango which then loads the UI allowing the users to construct a node
  graph & execute it in a once-only, throw-away operation which should be friendly & more powerful
  than attempting the same on the commandline.

- It should be possible to save the node graph to a file and run Mango as part of a shell scripting
  process with that file defining the operation you would like to perform.


## Goals

- It should be easier & quicker to accomplish easy & medium level data transformation in Mango than
  it would be to write similar Python or Javascript scripts to achieve the same result.

- It should be more intuitive & more powerful to use Mango for data transformations than to use
  commandline piping.


## Screenshot

![Mango Screenshot](/images/mango-screenshot.png?raw=true)

Basic example of extracting keys from a json object that comes in on standard-in. The keys are
written to standard-out.


## Related Work

- [jq](https://stedolan.github.io/jq/) can be used to quickly process JSON data with commandline
  based text expressions.


## Implemented Nodes

| **Name** | **From** | **To** |
| -------- | -------- | ------ |
| Standard in | - | StringArray |
| Standard out | * | - |
| Json parse | String | Json |
| Json stringify | Json | String |
| Json keys | Json | StringArray |
| Json object | StringArray + StringArray | Json |
| Lines | String | StringArray |
| String Contains | StringArray | StringArray |
| Sum | IntArray | Int |
| To Int | StringArray | IntArray |


## Potential Nodes

- File in
- File out
- CSV parse
- CSV stringify
- Json values
- Json pluck
- Json omit
- Key to value
- To float
- Add
- Substract
- String prepend
- String append
- Sort
- Randomise
- String slugify
- String camel case
- String snake case
- String title case
- String trim
- Constant
- Image resize
- Image greyscale
- Take
- Drop
- Union
- Difference


## Sub-Networks

To allow a custom series of operations within a single node that exposes expected input & output
connections.

