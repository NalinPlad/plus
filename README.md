# plus [+]

The only CLI calculator you'll ever need

Lots of additions to a basic `lalrpop` calculator parser including:

* Negative numbers 😱
* Variables 👩‍💻
* Trig functions 🌊
* Logarithms 🪵
* Absolute values 🧮
* Command history 📚
* Basic floating point rounding 👩‍🏫
* Exponents 📌
* Square, Cube roots 📦
* Pre-defined constants [pi,e] 🥧
* Ans variable, holds the answer to the previous operation 📝

And many more quality of life features

---

### Installation 🧮

Download prebuilt binaries in the 'releases' tab on the right
You can also add plus to the PATH for easy access




---

### Docs 📖

Ill add some basic documentation of some of plus's functionality here. Most people wont even need to read this to user plus, it is very straight forward!

#### Basic operations

plus's syntax is very similar to anything you would enter into a normal calculator(apart from a few special cases), so equtions like
```
plus> 5+5
10
plus> 13*3
39
plus> (16/4)^3
64
```

all work as intended. Note here that the parser will ignore whitespace, so you could also type `plus> ( 16  /4) ^  3`

#### Variables

Define variables with `=`

```
plus> x = 32
32
plus> y = 45/9
5
plus> z = x / y
6.4
```

then use it like this

```
plus> 3y / 2
7.5
plus> 5 * z
32
plus> 5(z)
32
plus> 5z
32
```

multiplication syntax is pretty robust, as seen in the last 3 lines

#### Trig functions

plus (currently) includes 6 trig functions, sin(), cos(), tan(), and csc(), sec(), cot(). It also defines the constant `pi`.

```
plus> sin(2pi)
0
plus> cos(32)
0.8342233605
plus> tan(180)
1.3386902104
plus> pi
3.1415926536
```

#### Roots

plus has a couple roots functions for calculating square and cube roots. Arbitrary roots are planned(See [TODO](https://github.com/NalinPlad/plus/blob/master/TODO)) Both `√` and `sqrt()` can be used

```
plus> √25
5
plus> sqrt(100)
10
plus> (3)√125
5
plus> cbrt(1000)
10
```
