# Zalgo

An esolang inspired by [Zalgo text](https://en.wikipedia.org/wiki/Zalgo_text)  
The esolangs.org page can be found [here](https://esolangs.org/wiki/Zalgo)

## Coding in Zalgo

A program in Zalgo is made up of clusters. A cluster is made up of 3 parts:

1. A centre character
	+ The character shouldn't be a whitespace character or combining diacritic.
	+ If it is then the interpreter will interpret the cluster as part of the previous cluster if there is one.
	+ If there isn't one then the cluster will be ignored.
	+ A centre character with no diacritics is ignored
	+ The values of the centre characters aren't used in the program so feel free to make them whatever you want!
2. A set of top diacritics
	+ This is any character in the Combining Diacritical Marks code block in Unicode that appears above the base character.
	+ Diacritics are added going from the centre outwards.
	+ The interpreter, however reads from top to bottom.
	+ Therefore, the top diacritics must be written **in the reverse order of execution**.
3. A set of bottom diacritics
	+ This is any character in the Combining Diacritical Marks code block in Unicode that appears below the base character.
	+ This part shouldn't be written in reverse as these diacritics are read in the same direction as they are added.

Clusters are executed in reading order.

## Zalgo instructions

### Top side instructions

| Character | Unicode Number    | Description                        |
|:---------:|:-----------------:|------------------------------------|
| `◌̀ - ◌̏`   | `U+0300 - U+030F` | Represents the hex digits `0 - F`. |
| `◌͆`       | `U+0346`          | Put at the start of the number for it's negative form |
| `◌̐`       | `U+0310`          | Pushes the hex value to the stack. If no hex value is given, it will push 0 |
| `◌̑`       | `U+0311`          | Pops the top value off the stack.  |

The interpreter treats hex digits that are next to each other as one single hex number.

### Bottom side instructions

| Character | Unicode Number    | Description                                                          |
|:---------:|:-----------------:|----------------------------------------------------------------------|
| `◌̥`       | `U+0325`          | Pops a value off the stack, if it's 0, skip the next cluster.        |
| `◌̝`       | `U+031D`          | Pops a value off the stack and prints its Unicode character.             |
| `◌̞`       | `U+031E`          | Input works with a buffer. The command takes the first character from the buffer and pushes it to the stack. If the buffer is empty, the user is prompted to input something to set the buffer to before the process continues. All inputs have a null byte added to the end. |
| `◌͈`       | `U+0348`          | Duplicates the top value of the stack.                               |
| `◌͍`       | `U+034D`          | Pops a value off the stack and jumps to the nth cluster (0 indexed). Any remaining instructions in the current cluster will be ignored. |
| `◌̟`       | `U+031F`          | Pops a value off the stack X and another Y, push X+Y.                |
| `◌̠`       | `U+0320`          | Pops a value off the stack X and another Y, push X-Y. |
| `◌͓`       | `U+0353`          | Pops a value off the stack X and another Y, push X*Y.                |
| `◌̡`       | `U+0321`          | Pops a value off the stack X and another Y, push Y/X (rounded down). |
| `◌̢`       | `U+0322`          | Pops a value off the stack X and another Y, push Y%X.                |
| `◌̭`       | `U+032D`          | Pops a value off the stack X and another Y, push X∧Y.                |
| `◌̬`       | `U+032C`          | Pops a value off the stack X and another Y, push X∨Y.                |
| `◌͉`       | `U+0349`          | Pops a value off the stack X and inverts the significant bits (NOT gate). |
| `◌̜`       | `U+031C`          | Pops a value off the stack X and another Y, push Y << X.             |
| `◌̹`       | `U+0339`          | Pops a value off the stack X and another Y, push Y >> X.             |
| `◌̳`       | `U+0333`          | Pops a value off the stack X and another Y. Push 1 if X == Y or 0 otherwise. |
| `◌͕`       | `U+0355`          | Pops a value off the stack X and another Y. Push 1 if Y > X or 0 otherwise. |
| `◌͔`       | `U+0354`          | Pops a value off the stack X and another Y. Push 1 if Y < X or 0 otherwise. |
| `◌̙`       | `U+0319`          | Pops a value off the stack X and another Y. Cycle the top Y elements of the stack by X. (Positive means cycle up, negative means cycle down)|


## Running the interpreter

[Install rust](https://www.rust-lang.org/tools/install) if it isn't on your system.

```console
$ cargo run [filename]
```
