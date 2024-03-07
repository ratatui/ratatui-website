---
title: "How to: test UI Layout"
---

Testing is important in writing effective code. It helps us realize if the code we are writing is doing what we expect it to do. Let's write some tests for the UIs we created in preceding chapters.

While the writing the test, keep this in mind:  
1. Since the direction of layout was `Vertical` (|), only the height of the subsequent `Rect`s and `y` intercept will be affected.
2. Terminal can be rendered on a test backend.
3. Express the type of the variable so that you don't get confused when you get back to it later.
4. Don't forget to add `use::rc::Rc` at the top of your file.

## Writing a test for the dynamic layout

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, prelude::Rect, Terminal};

    #[test]
    fn test_get_layout_based_on_messages() {
        let backend: TestBackend = TestBackend::new(10, 10);
        let mut terminal: Terminal<TestBackend> = Terminal::new(backend).unwrap();

        let msg_count: usize = 50;
        let frame: Frame<'_> = terminal.get_frame();
        let actual_layout: Rc<[Rect]> = get_layout_based_on_messages(msg_count, &frame);

        let expected_layout: Rc<[Rect]> = [
            Rect {
                x: 0,
                y: 0,
                width: 10,
                height: 5,
            },
            Rect {
                x: 0,
                y: 5,
                width: 10,
                height: 5,
            },
        ]
        .into();

        assert_eq!(
            expected_layout, actual_layout,
            "failed for message count {}",
            msg_count
        );

        let msg_count: usize = 60;
        let actual_layout: Rc<[Rect]> = get_layout_based_on_messages(msg_count, &frame);

        let expected_layout: Rc<[Rect]> = [
            Rect {
                x: 0,
                y: 0,
                width: 10,
                height: 8,
            },
            Rect {
                x: 0,
                y: 8,
                width: 10,
                height: 2,
            },
        ]
        .into();

        assert_eq!(
            expected_layout, actual_layout,
            "failed for message count {}",
            msg_count
        );
    }
}
```

## Writing a test for `centered_rect`

```rust
    #[test]
    fn test_centered_rect() {
        let rect = Rect {
            x: 0,
            y: 0,
            width: 200,
            height: 150,
        };
        let expected_rect = Rect {
            x: 74,
            y: 18,
            width: 50,
            height: 113,
        };
        let actual_rect = centered_rect(rect, 25, 75);
        assert_eq!(actual_rect, expected_rect);
    }
```