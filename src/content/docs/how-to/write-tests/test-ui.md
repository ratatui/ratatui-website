---
title: "How to: test UI"
---

Testing is essential for creating effective, error-free, and secure code. It verifies that our code behaves as expected.  
Consider you've developed a helper function that renders a Rect based on a specified x and y intercept percentage of another Rect.

```rust
{{#include @code/how-to-centre-a-rect/src/main.rs:centered_rect}}
```
You want to check the value of the resulting centered `Rect` with the value of an expected `Rect`.  
When writing the expect_rect remember:
Since the direction of layout was `Vertical` - `(|)`, only the height of the subsequent `Rect`s and `y` intercept will be affected.

```rust
{{#include @code/how-to-centre-a-rect/src/main.rs:test_function}}
```
