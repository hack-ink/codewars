## Detail

[Remove duplicate words](https://www.codewars.com/kata/remove-duplicate-words/rust)

Your task is to remove all duplicate words from string, leaving only single words entries.

Example:

Input:

'alpha beta beta gamma gamma gamma delta alpha beta beta gamma gamma gamma delta'

Output:

'alpha beta gamma delta’

## Thinking

Just use `contains()`.

There’re two bugs till *7/22/2018*:

1. You must add `use std::collections::HashSet` even you haven’t use `HashSet`.
2. The return type should be `String` rather `&str`.