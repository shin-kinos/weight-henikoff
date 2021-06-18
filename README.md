# weight-henikoff
A small program calculates sequence weighting in MSA using Position Based Method.

## Description
* It calculates weighting scores in MSA to reduce sequence redundancy using Position Based Method by Henikoff-Henikoff[1].

## How to compile
You can compile this program with `Cargo`🦀📦.

[e.g.]

```
% cd weight-henikoff-main
% cargo build --release
```

## Input file format
Aligned Multi-FASTA format in amino acid sequences.

See some example files in `demo` directory.

## Usage

Options :
* `-i` : Input file name.
* `-t` : Tolerate AA types 'B', 'Z', 'X' and 'U' in input file.

[e.g.]

```
% ./weight-henikoff -i input.fasta -t yes
```

Type `-h` to see a help.

## Result
Number `\t` Weighting score `\t` Name of the sequence (the title line)

## References
1. Henikoff, Steven, and Jorja G. Henikoff. "Position-based sequence weights." Journal of molecular biology 243.4 (1994): 574-578.
