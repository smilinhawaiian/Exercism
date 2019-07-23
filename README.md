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
I used match and if/else blocks, to check for numbers, letters, and case, 
but I there may be a better or more efficient solution.   
https://exercism.io/my/solutions/9120a90cc6aa4f49b26f0f95e0bfd896  

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
I used mod to figure out if n was odd or even, and a count to keep track
of the number of steps, while recursively calling the collatz function.    
For any number less han one, None shouldbe returned.  
https://exercism.io/my/solutions/0d096db6b9604d6099be9afb9c984179  

_From Readme:_   
The Collatz Conjecture or 3x+1 problem can be summarized as follows:  

Take any positive integer n. If n is even, divide n by 2 to get n / 2. If n is
odd, multiply n by 3 and add 1 to get 3n + 1. Repeat the process indefinitely.
The conjecture states that no matter which number you start with, you will
always reach 1 eventually.  

Given a number n, return the number of steps required to reach 1.  

#### Examples  
Starting with n = 12, the steps would be as follows:

0. 12
1. 6
2. 3
3. 10
4. 5
5. 16
6. 8
7. 4
8. 2
9. 1

Resulting in 9 steps. So for input n = 12, the return value would be 9.  


### Saddle-points  
_Learning Summary:_     
*iterators - vectors*  


_From Readme:_   
Detect saddle points in a matrix.

So say you have a matrix like so:

```text
    1  2  3
  |---------
1 | 9  8  7
2 | 5  3  2     <--- saddle point at column 1, row 2, with value 5
3 | 6  6  7
```

It has a saddle point at column 1, row 2.

It's called a "saddle point" because it is greater than or equal to
every element in its row and less than or equal to every element in
its column.

A matrix may have zero or more saddle points.

Your code should be able to provide the (possibly empty) list of all the
saddle points for any given matrix.

The matrix can have a different number of rows and columns (Non square).

Note that you may find other definitions of matrix saddle points online,
but the tests for this exercise follow the above unambiguous definition.

#### Efficiency Notice  
This exercise uses a _vector of vectors_ to store the content of matrices. While
this exercise is designed to help students understand basic concepts about
vectors, such as indexing, and that nested data types are legal, _vector of
vectors_ is a suboptimal choice for high-performance matrix algebra and any
similar efficient processing of larger amounts of data.  


### Isogram  
_Learning Summary:_  
*chars - iterators - strings*  
An Iterator was helpful in this exercise to dissect the input word.  
I used a HashSet to store and check for duplicate letters.  
Notes:  
An empty string is an isogram.  
Upper and lowercase characters of the same type are considered to be the same letter.  
https://exercism.io/my/solutions/79fa31189e6742d0ac26fabb40ced6fd  

_From Readme:_   
Determine if a word or phrase is an isogram.

An isogram (also known as a "nonpattern word") is a word or phrase without a repeating letter, 
however spaces and hyphens are allowed to appear multiple times.

Examples of isograms:

- lumberjacks
- background
- downstream
- six-year-old

The word *isograms*, however, is not an isogram, because the s repeats.  


### Say  
_Learning Summary:_  
*modulus - string concatenation*  


_From Readme:_   
Given a number from 0 to 999,999,999,999, spell out that number in English.

#### Step 1

Handle the basic case of 0 through 99.

If the input to the program is `22`, then the output should be
`'twenty-two'`.

Your program should complain loudly if given a number outside the
blessed range.

Some good test cases for this program are:

- 0
- 14
- 50
- 98
- -1
- 100

##### Extension

If you're on a Mac, shell out to Mac OS X's `say` program to talk out
loud. If you're on Linux or Windows, eSpeakNG may be available with the command `espeak`.

#### Step 2

Implement breaking a number up into chunks of thousands.

So `1234567890` should yield a list like 1, 234, 567, and 890, while the
far simpler `1000` should yield just 1 and 0.

The program must also report any values that are out of range.

#### Step 3

Now handle inserting the appropriate scale word between those chunks.

So `1234567890` should yield `'1 billion 234 million 567 thousand 890'`

The program must also report any values that are out of range.  It's
fine to stop at "trillion".

#### Step 4

Put it all together to get nothing but plain English.

`12345` should give `twelve thousand three hundred forty-five`.

The program must also report any values that are out of range.

##### Extensions

Use _and_ (correctly) when spelling out the number in English:

- 14 becomes "fourteen".
- 100 becomes "one hundred".
- 120 becomes "one hundred and twenty".
- 1002 becomes "one thousand and two".
- 1323 becomes "one thousand three hundred and twenty-three".


#### Rust Specific Exercise Notes

This is slightly changed in the Rust version, compared to other
language versions of this exercise.  Instead of requiring you to return
errors for out of range, we are using Rust's strong type system to limit
input.  It is much easier to make a function deal with all valid inputs,
rather than requiring the user of your module to handle errors.

There is a -1 version of a test case, but it is commented out.
If your function is implemented properly, the -1 test case should not compile.

Adding 'and' into number text has not been implemented in test cases.

##### Extension

Add capability of converting up to the max value for u64: 9,223,372,036,854,775,807.

For hints at the output this should have, look at the last test case.  



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
