## Partial Move

Within the destructuring of a single variable, both by-move and by-reference
pattern bindings can be used at the same time. Doing this will result in a
partial move of the variable, which means that parts of the variable will be
moved while other parts stay. In such a case, the parent variable cannot be used
afterwards as a whole, however the parts that are only referenced (and not
moved) can still be used.
