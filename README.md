
# Mango

Mango is designed to be a quick & easy tool for processing data use node networks. The idea is to
provide common operations in a more powerful, flexible & discoverable way than the commandline.
Simple data typing along with easy to find, useful transformations make it an ideal tool for quickly
messing around with data.

## How to use

1.  Clone this repository
2.  Run `cargo build`
3.  Run `cat ./examples/json | ./target/debug/mango new`
4.  Press 'i' to insert the first node, this opens a text-prompt.
5.  Type 'standard-in' and press enter to create the node. This will also select the node.
6.  Press 'a' to add a node after the selected node. Type 'json-parse' and enter.
7.  Press 'a' to add another node. Type 'standard-out' and enter'.
8.  Press 'i' to insert a node between the current node & its input. Type 'json-keys' and enter.
9.  Press 'q' to exit and run the node graph.
10. You should see:
    ```
    simple
    secondkey
    ```
    Printed to standard out. These are the two keys in the json file.


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


## Technology

Mango is written in Rust & uses the [Conrod](https://github.com/PistonDevelopers/conrod) framework
for the 2D graphics. It is possible that it'll be served better by a different framework or a
combination of Conrod for the node graph & a more traditional GUI framework for the rest.


## Screenshot

![Mango Screenshot](/images/mango-screenshot.png?raw=true)

Basic example of extracting keys from a json object that comes in on standard-in. The keys are
written to standard-out.


## Related Work

- [jq](https://stedolan.github.io/jq/) can be used to quickly process JSON data with commandline
  based text expressions.


## Controls

The interface is largely keyboard driven & inspired by vim key bindings. See the 'How to use'
section for a typical session example.


### Keys

| **Key** | **Function** |
| ------- | ------------ |
| i | Insert new node as mouse position if nothing is selected. Inserts as input to the currently
selected node. Inserts a node between the current node & its input if it has one. |
| a | Adds a node after the currently selected node. Insert a node between the currently select node
& its output if it has one. |
| s | Replaces the currently selected node with a new node, wiring in any inputs & outputs that were
present. |
| q | Exits and runs the node graph |
| u | Undoes the last action |
| r | Redoes the last undone action |
| h | Moves the selection from the current node to its input |
| l | Moves the selection from the current nodes to its output |
| : | Start a command prompt at the bottom that has only one acceptable command `:w <filename>`
which writes the current nodes graph to the specified file |


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

