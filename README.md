# fiveletterwords

Some implementations of the  ***five five-letter words with twenty-five unique letters*** problem described in this 
video https://www.youtube.com/watch?v=_-AfhLQfb6w and for which a solution is : jumpy, chivw, grosz, fldxt, baken

The objective is to compare how different programming languages implement the same algorithm, especially
regarding code readability, conciseness and performances.

### Algorithm

First of all the dictionary of 5 letter English words is filtered to keep only the words having non-unique letters.

Then the algorithm uses the *divide and conquer* principle to find the solution in a minimum of time: the words set is
split into 2 parts: a set containing a given letter (example "e") and a set not containing this letter.

The letter is not chosen randomly or alphabetically, but is the most frequent english letter that is not yet part of the 
partial solution (set of word satisfying the condition). This allows a better distribution between the sets 

Then for each word in the first set, we concatenate the word with the partial solution, filter out all the words from the 
second set that has a duplicate letter with this concatenation, and for the other ones call the same function recursively.

The algorithm stops whenever a partial solution has 25 letters. 


### Kotlin

The code is short, easy to read. The performances are quite good (~1500ms)

### Scala

The code is also short but a bit trickier to understand. The performances are acceptable (~5600 ms)

### Rust

Being a newbie, the code may not be perfect..

First the code was extremely slow due to usage of Strings and method `contain` which does not seem
to be optimized (around 30 sec total execution time).

Then implementing Words as arrays of bytes dramatically increased the speed which is around 7ms total !
(the slower parts being reading the file)

### Python

Code is quite straightforward. Execution time is around 30 sec which is somehow expected
