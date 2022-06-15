# Zalgo

An esolang inspired by [Zalgo text](https://en.wikipedia.org/wiki/Zalgo_text)

## Coding in Zalgo

A program in Zalgo is made up of clusters. A cluster is made up of 3 parts:

1. A centre character
	+ The character shouldn't be a whitespace character or combining diacritic.
	+ If it is then the interpreter will interpret the cluster as part of the previous cluster if there is one.
	+ If there isn't one then the cluster will be ignored.
	+ There values of the centre characters aren't used in the program so feel free to make them whatever you want!
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

(Zalgo is unfinished and more instructions will be added)

### Top side instructions

| Character | Unicode Number    | Description                       |
|:---------:|:-----------------:|-----------------------------------|
| `◌̀ - ◌̏`   | `U+0300 - U+030F` | Represents the hex digits `0 - F` |
| `◌̐`       | `U+0310`          | Pushes the hex value to the stack |

The interpreter treats hex digits that are next to each other as one single hex number.

### Bottom side instructions

| Character | Unicode Number    | Description                                             |
|:---------:|:-----------------:|---------------------------------------------------------|
| `◌̝`       | `U+031D`          | Pops a value of the stack and prints it's Unicode value |
