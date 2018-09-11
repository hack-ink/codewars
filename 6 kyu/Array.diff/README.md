## Detail
[Array.diff](https://www.codewars.com/kata/array-dot-diff/train/rust)
Your goal in this kata is to implement a difference function, which subtracts one list from another and returns the result.

It should remove all values from list `a`, which are present in list `b`.

```javascript
array_diff([1,2],[1]) == [2]
```
```ruby
array_diff([1,2],[1]) == [2]
```
```python
array_diff([1,2],[1]) == [2]
```
```coffeescript
array_diff([1,2],[1]) == [2]
```
```haskell
difference [1,2] [1] == [2]
```
```csharp
Kata.ArrayDiff(new int[] {1, 2}, new int[] {1}) => new int[] {2}
```
```rust
array_diff(vec![1,2], vec![1]) == vec![2]
```
```clojure
(= (array-diff [1 2] [1]) [2])
```


If a value is present in `b`, all of its occurrences must be removed from the other:

```javascript
array_diff([1,2,2,2,3],[2]) == [1,3]
```
```ruby
array_diff([1,2],[1]) == [2]
```
```python
array_diff([1,2,2,2,3],[2]) == [1,3]
```
```coffeescript
array_diff([1,2,2,2,3],[2]) == [1,3]
```
```haskell
difference [1,2,2,2,3] [2] == [1,3]
```
```csharp
Kata.ArrayDiff(new int[] {1, 2, 2, 2, 3}, new int[] {2}) => new int[] {1, 3}
```
```rust
array_diff(vec![1,2,2,2,3], vec![2]) == vec![1,3]
```
```clojure
(= (array-diff [1,2,2,2,3] [2]) [1,3])
```
## Thinking

Try `a.retain()`.