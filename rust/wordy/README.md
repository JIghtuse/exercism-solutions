# Wordy

Write a program that takes a word problem and returns the answer as an integer.

## Step 1

E.g.

> What is 5 plus 13?

The program should handle large numbers and negative numbers.

Remember, that these are verbal word problems, not treated as you
normally would treat a written problem.  This means that you calculate
as you move forward each step.  In other words, you should ignore order
of operations.  3 + 2 * 3 = 15, not 9.

Use the tests to drive your solution by deleting the `skip` in one test
at a time.

## Step 2

E.g.

> What is 5 plus 13?

> What is 7 minus 5?

> What is 6 multiplied by 4?

> What is 25 divided by 5?

## Step 3

E.g.

> What is 5 plus 13 plus 6?

> What is 7 minus 5 minus 1?

> What is 9 minus 3 plus 5?

> What is 3 plus 5 minus 8?

## Step 4

E.g.

> What is 5 plus 13?

> What is 7 minus 5?

> What is 6 times 4?

> What is 25 divided by 5?

> What is 78 plus 5 minus 3?

> What is 18 times 3 plus 16?

> What is 4 times 3 divided by 6?

> What is 4 plus 3 times 2?

## Extensions

Implement questions of the type:

> What is 2 raised to the 5th power?

Remember to write failing tests for this code.

## Rust Installation

Refer to the [exercism help page][help-page] for Rust installation and learning
resources.

## Writing the Code

Execute the tests with:

```bash
$ cargo test
```

All but the first test have been ignored.  After you get the first test to
pass, remove the ignore flag (`#[ignore]`) from the next test and get the tests
to pass again.  The test file is located in the `tests` directory.   You can
also remove the ignore flag from all the tests to get them to run all at once
if you wish.

Make sure to read the [Crates and Modules](https://doc.rust-lang.org/stable/book/crates-and-modules.html) chapter if you
haven't already, it will help you with organizing your files.

## Feedback, Issues, Pull Requests

The [exercism/xrust](https://github.com/exercism/xrust) repository on GitHub is the home for all of the Rust exercises. If you have feedback about an exercise, or want to help implement new exercises, head over there and create an issue. Members of the [rust track team](https://github.com/orgs/exercism/teams/rust) are happy to help!

If you want to know more about Exercism, take a look at the [contribution guide](https://github.com/exercism/x-common/blob/master/CONTRIBUTING.md).

[help-page]: http://exercism.io/languages/rust
[crates-and-modules]: http://doc.rust-lang.org/stable/book/crates-and-modules.html

## Source

Inspired by one of the generated questions in the Extreme Startup game. [https://github.com/rchatley/extreme_startup](https://github.com/rchatley/extreme_startup)
