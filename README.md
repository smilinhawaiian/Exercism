# Exercism.io Assignment  
Sharice Mayer
July 2019

## Summary:  

### Hello World  
_Summary:_  
*println*  
The test`assert_eq!("Hello, World!", hello_world::hello())`;
failed until fixing the output of the function to be the same.  
https://exercism.io/my/solutions/970e3c789fe8478aa3d393868b544125  

_From Readme:_
The classical introductory exercise. Just say "Hello, World!".

The objectives are simple:

- Write a function that returns the string "Hello, World!".
- Run the test suite and make sure that it succeeds.
- Submit your solution and check it at the website.  


### Leap  
_Learning Summary:_  
*booleans - conditionals*  
Used arithmetic remainder operator to test for divisibility.  
Needed to cover all cases using booleans.  
https://exercism.io/my/solutions/58d95530589149c1acf8e7ee3fda3a63  

_From Readme:_   
Given a year, report if it is a leap year.

The tricky thing here is that a leap year in the Gregorian calendar occurs:

```text
on every year that is evenly divisible by 4
  except every year that is evenly divisible by 100
    unless the year is also evenly divisible by 400
```

For example, 1997 is not a leap year, but 1996 is.  1900 is not a leap
year, but 2000 is.  


### Proverb  
_Learning Summary:_  
*format*  
Checked for list passed in, got list length, and created last line from first arg.  
Then iterated over list from 1..length adding a single line at a time
until the end was found, adding the last line at that point.  
Learned that to add strings, must add String + &str = String  
https://exercism.io/my/solutions/28829b2a85414b5cb99769212375033b  

_From Readme:_   
For want of a horseshoe nail, a kingdom was lost, or so the saying goes.

Given a list of inputs, generate the relevant proverb. For example, given the list `["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"]`, you will output the full text of this proverbial rhyme:

```text
For want of a nail the shoe was lost.
For want of a shoe the horse was lost.
For want of a horse the rider was lost.
For want of a rider the message was lost.
For want of a message the battle was lost.
For want of a battle the kingdom was lost.
And all for the want of a nail.
```

Note that the list of inputs may vary; your solution should be able to handle lists of arbitrary length and content. No line of the output text should be a static, unchanging string; all should vary according to the input given.  


### Bob  
_Learning Summary:_  
*chars - string functions*  
This exercise took several possible cases into account for the result.
I used match and if/else blocks, but I know there is a better/
more efficient solution. 

_From Readme:_   
Bob is a lackadaisical teenager. In conversation, his responses are very limited.

Bob answers 'Sure.' if you ask him a question, such as "How are you?".

He answers 'Whoa, chill out!' if you YELL AT HIM (in all capitals).

He answers 'Calm down, I know what I'm doing!' if you yell a question at him.

He says 'Fine. Be that way!' if you address him without actually saying
anything.

He answers 'Whatever.' to anything else.

Bob's conversational partner is a purist when it comes to written communication
and always follows normal rules regarding sentence punctuation in English.  


### Collatz-conjecture  
_Learning Summary:_  
*option - math*  


_From Readme:_   


### Saddle-points  
_Learning Summary:_     
*iterators - vectors*  


_From Readme:_   


### Isogram  
_Learning Summary:_  
*chars - iterators - strings*  


_From Readme:_   


### Say  
_Learning Summary:_  
*modulus - string concatenation*  


_From Readme:_   
some  


## Writing the Code

Execute the tests with:

```bash
$ cargo test
```

All but the first test have been ignored. After you get the first test to
pass, open the tests source file which is located in the `tests` directory
and remove the `#[ignore]` flag from the next test and get the tests to pass
again. Each separate test is a function with `#[test]` flag above it.
Continue, until you pass every test.

If you wish to run all tests without editing the tests source file, use:

```bash
$ cargo test -- --ignored
```

To run a specific test, for example `some_test`, you can use:

```bash
$ cargo test some_test
```

If the specific test is ignored use:

```bash
$ cargo test some_test -- --ignored
```

To learn more about Rust tests refer to the [online test documentation][rust-tests]

Make sure to read the [Modules][modules] chapter if you
haven't already, it will help you with organizing your files.

## Further improvements

To format your solution, inside the solution directory use

```bash
cargo fmt
```

To see, if your solution contains some common ineffective use cases, inside the solution directory use

```bash
cargo clippy --all-targets
```

## Submitting the solution

Generally you should submit all files in which you implemented your solution (`src/lib.rs` in most cases). If you are using any external crates, please consider submitting the `Cargo.toml` file. This will make the review process faster and clearer.

## Feedback, Issues, Pull Requests

The [exercism/rust](https://github.com/exercism/rust) repository on GitHub is the home for all of the Rust exercises. If you have feedback about an exercise, or want to help implement new exercises, head over there and create an issue. Members of the rust track team are happy to help!

If you want to know more about Exercism, take a look at the [contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## Source

This is an exercise to introduce users to using Exercism [http://en.wikipedia.org/wiki/%22Hello,_world!%22_program](http://en.wikipedia.org/wiki/%22Hello,_world!%22_program)

## Submitting Incomplete Solutions
It's possible to submit an incomplete solution so you can see how others have completed the exercise.
