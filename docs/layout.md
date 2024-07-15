# Layout

## Goals

- **Simple** - It should be easy to understand and predict
- **No special cases** - I don't want many special cased if statements in the code, if the result is really bad I would rather try to improve the whole algorithm in general, but the boundaries here are blurry.
- **Easy to read** - Size isn't a problem here. I prefer to have a wider diagram where the logic mostly flows from left to right and not turn around multiple times just to be stuffed together.

## Fragmentation

This process turns the graph into fragments or blocks which will be related to each other in a tree structure.
This is done because the sizes of the fragments can later be easier added together when traversing a tree.

Everytime a branch occurs / the flow isn't linear anymore, the following logic should be in a new fragment which the current block holds.

## Arrangement

Inside a fragment, it's children are vertically sorted by it's distance to both the start and end of the parent.
Horizontally, they are spread out as wide as they can in the minimal size of the parent.
