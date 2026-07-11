<h1 align="center">BottomSpeak</h1>

<div align="center">

![gender? I hardly know 'er](https://pride-badges.pony.workers.dev/static/v1?label=gender%3F+i+hardly+know+%27er&labelColor=%23555&stripeWidth=8&stripeColors=FCF434%2CFFFFFF%2C9C59D1%2C2C2C2C)
![trans rights :3](https://pride-badges.pony.workers.dev/static/v1?label=trans+rights+%3A3&labelColor=%23555&stripeWidth=8&stripeColors=5BCEFA%2CF5A9B8%2CFFFFFF%2CF5A9B8%2C5BCEFA)
![women and enbies pretty](https://pride-badges.pony.workers.dev/static/v1?label=women+and+enbies+pretty&labelColor=%23555&stripeWidth=8&stripeColors=D52D00%2CEF7627%2CFF9A56%2CFFFFFF%2CD162A4%2CB55690%2CA30262)

</div>

BottomSpeak is the ultimate language for expressing yourself through programming. Expressions? Boring. Types? Who needs 'em? Now you can program with the same kind of basic symbols you'd use when faced with even the slightest amount of dominance!

BottomSpeak is a stack-based language and as such has a fairly simple instruction set based around manipulating said stack. BottomSpeak does not require a specific file extension, so feel free to use whatever you like, or none at all! All examples in the repo will use the `.uwu` extension, simply because why not?

If you encounter any problems using the language, don't hesitate to [open an issue](https://github.com/oughtum/bottomspeak/issues/new) in the repo!

## Language Syntax

### Keysmashes

These are how you push values to the stack. They can have a maximum length of 128 characters and consist purely of standard ASCII alphabet characters (a-zA-Z). The length of a keysmash is encoded as a byte value in the range 0-127 for lowercase keysmashes and 128-255 for uppercase ones.

For example, `asdfglaskdjh` has a length of 12 characters and thus is encoded as the value 11 in decimal because lowercase keysmashes start at 0, while `AKSDJHFPAOISDUJASDLKFJOI` has a length of 24 characters and thus is encoded as the value 151 because uppercase keysmashes start at 128.

Ordinarily, keysmashes must be separated by whitespace or a different case, otherwise they get parsed as a single keysmash, but it's also possible to use semicolons (`;`) to delimit them as they are also a common keysmash character e.g. `asdlkj;dfkgjhasdo` is equivalent to `asdlkj dfkgjhasdo`, also `sdlkjBAKLSDJ` and `sdlkj;BAKLSDJ` are equivalent to `sdlkj BAKLSDJ`. In fact, it's possible to use semicolons to delimit any and all instructions as they are just treated the same as whitespace, but their intended use is for keysmashes.

If a keysmash ends with `~`, it instead pushes the byte to the scratchpad, which is a temporary storage buffer that exists outside of the stack and its value can be retrieved and pushed back to the stack at any time. The scratchpad can contain only a single byte and if it already has a value, pushing will overwrite the existing value.

### Instructions

#### Arithmetic

- `:3` - Pops the last two values on the stack, adds them together and pushes the result back to the stack. Multiple values can be added together at once by simply repeating the `3` e.g. a stack with values `[1, 5, 8, 2, 9]` followed by the instruction `:333` would result in the stack `[1, 24]`.
- `>:3` - Identical to `:3` except used for subtraction.

Both instructions allow overflow behaviour i.e. 0 - 1 == 255 and 255 + 1 == 0.

#### Conditional Evaluation

These instructions look at the top two stack values [`a`, `b`], then check a boolean predicate and skip the next instruction if the result is falsey. These are the predicates checked for each instruction:

- `^x^` - `a == b`
- `^o^` - `a > b`
- `^w^` - `a < b`
- `>x<` - `a != b`
- `>o<` - `a >= b`
- `>w<` - `a <= b`

#### Moving Stack Values Around

- `>//<` - Duplicates the value at the top of the stack. The value can be duplicated multiple times just like with `:3` by repeating the `//` e.g. `>//////<` would duplicate the value three times.
- `>\\<` - Identical to `>//<` except duplicates the top two stack values i.e. the stack `[1, 2]` would become `[1, 2, 1, 2]`.
- `@~@` - Swaps the top two stack values.
- `O~O` - Rotates through the last three stack values i.e. `[1, 2, 3]` would become `[2, 3, 1]` and then `[3, 1, 2]`, before returning to `[1, 2, 3]`.
- `0~0` - Flips the whole stack i.e. `[1, 2, 3]` would become `[3, 2, 1]`.
- `UwU` - Pops the top value from the stack and discards it.
- `OwO` - Pops the value in the scratchpad and pushes it to the stack.

#### I/O

- `>~<` - Takes in a single byte of input from the commandline via the `-i/--input` flag. Multiple byte values can be provided at once but each use of this instruction will only pop the last input value at a time.
- `meow` - One of several [default keywords](#customisation) used for popping the top stack value and printing it as an ASCII character to stdout. Any keywords defined via the `BOTTOMSPEAK_PRINT_KEYWORDS` environment variable are valid as a print instruction.
- `meow!` - Identical to regular printing except it prints the literal byte value on the stack e.g. `[104]` would print `104` instead of `h` which has the ASCII code 104.
- `meow~` - Pops the last three stack values, pads them with an extra zero byte & constructs a unicode codepoint to be printed. For example, a stack with values `[1, 249, 122]` would be `[0x01, 0xf9, 0x7a]` in hexadecimal (or `[0x00, 0x01, 0xf9, 0x7a]` with padding), and thus is converted to the unicode codepoint `u1f97a` (🥺).
- `mommy` - Another [default keyword](#customisation) normally used internally by the interpreter when reporting errors but also allows users of the language to debug the stack by printing it to stdout. Any keywords defined via the `BOTTOMSPEAK_INTERP_TITLE` environment variable are valid as a debug print instruction. This instruction will also print out the scratchpad value e.g. `[0]:[1, 2, 3]`, where `[0]` is the scratchpad and `[1, 2, 3]` is the main stack.
- `mommy~` - Identical to regular stack printing but instead pretty prints the stack, this just means the output string is expanded to span over newlines instead of being a compact single line as with debug printing.
- `🏳️‍🌈` - Pops the last four bytes on the stack to construct a set of ANSI escape sequences for printing styled text. The role of each byte is as follows:
  - The first byte is the byte value of the character to print.
  - The second byte is the ANSI 256 colour code to use for the foreground colour.
  - The third byte is the ANSI 256 colour code to use for the background colour.
  - The last byte is a set of bitflags corresponding to the following modifiers:
    - 0 - No Modifiers
    - 1 - Bold
    - 2 - Dim
    - 4 - Italic
    - 8 - Underlined
    - 16 - Blink
    - 32 - Reverse
    - 64 - Hidden
    - 128 - Strikethrough

    So, for example, a stack with the values `[38, 3, 1, 13]` would result in the `&` character being printed bold, italic, underlined, and with a yellow foreground colour and a red background colour.

### Subroutines

Subroutines are a way to reuse code and as we all know, subs love getting used over and over again, so in order to define one, simply type what you'd normally type if someone told you that you were a good \<insert term here\>, like so:

```
alsdkfkl 🥺
```

To return from a subroutine, simply use `>.<`. With [conditional evaluation](#conditional-evaluation), it is possible to return early, however all subroutines must still contain a final return because you've always got to make sure your subs finish eventually.

In order to get a subroutine to do anything, you can jump to its instruction block by specifying the identifier of the subroutine you wish to jump to and then `👉👈`, like so:

```
alsdkfkl 👉👈
```

### Comments

Comments are always inline and can be formed using the trans flag emoji '🏳️‍⚧️':

```
🏳️‍⚧️ Trans rights are human rights :3
```

## Error Reporting

BottomSpeak is a silly language but that doesn't mean it can't have good error reporting! The language shows you exactly where you went wrong and also gives some supportive words of encouragement, for example:

```
Mommy found some errors in your code but it's okay, sweetie, mommy believes in you <3

error[E0002]: I know speaking is hard for my good girl but could you please add a `<` at the end for me, sweetheart~
  ╭─ bleh.uwu:1:13
  │
1 │ haiiii bweh >w
  │             ^^
2 │
```

Now you can finally get some validation for all your hard work!

## Examples

Here is a simple "Hello, world!" program with comments tracking the state of the stack and each time text is printed:

```
🏳️‍⚧️ prints "Hello, world!" to stdout
haiii 🥺                                   🏳️‍⚧️ []
  asdelkjla;afsdlkfjaksldkfjd >//////<     🏳️‍⚧️ [8] -> [8, 16] -> [8, 16, 16, 16, 16]
  :3333 >//< mreow                         🏳️‍⚧️ [72] -> [72, 72] -> [72] 'H'
  dksfhapsduiofalsdkfjalsdkasdj :3 >//<    🏳️‍⚧️ [72, 28] -> [100] -> [100, 100]
  aa :3 mreow >//<                         🏳️‍⚧️ [100, 100, 1] -> [100, 101] -> [100] 'e' -> [100, 100]
  asdfkjhsl :3 >////< meow mrrp            🏳️‍⚧️ [100, 100, 8] -> [100, 108] -> [100, 108, 108, 108] 'l' 'l' -> [100, 108]
  waow :3 yip                              🏳️‍⚧️ [100, 108, 3] -> [100, 111] 'o' -> [100]
  asdkljfhasdklufhasdfasdhjflkjahld >////< 🏳️‍⚧️ [100, 32] -> [100, 32, 32, 32]
  wlsdkjfhaioua :3 meow mrrp @~@ >//<      🏳️‍⚧️ [100, 32, 32, 32, 12] -> [100, 32, 32, 44] -> [100, 32] ',' ' ' -> [32, 100] -> [32, 100, 100]
  fsdklfakl :3 >//<                        🏳️‍⚧️ [32, 100, 100, 8] -> [32, 100, 108] -> [32, 100, 108, 108]
  mkasdpfoasik :3 yip >//<                 🏳️‍⚧️ [32, 100, 108, 108, 11] -> [32, 100, 108, 119] 'W' -> [32, 100, 108] -> [32, 100, 108, 108]
  bleh :3 mreow >//<                       🏳️‍⚧️ [32, 100, 108, 108, 3] -> [32, 100, 108, 111] -> [32, 100, 108] 'o' -> [32, 100, 108, 108]
  asdlkvj :3 meow mrrp mreow               🏳️‍⚧️ [32, 100, 108, 108, 6] -> [32, 100, 108, 114] -> [32] 'r' 'l' 'd'
  um :3 mrrp >.<                           🏳️‍⚧️ [32, 1] -> [33] '!' -> []

haiii 👉👈
```

As with any language, there is no singular solution to a problem, so your "Hello, world!" could look very different to this one.

You can find the above example within the `examples/` directory which is currently a little barebones. Feel free to submit a pull request with more examples!

## Customisation

There are several behaviours which can be customised through environment variables.

- `BOTTOMSPEAK_INTERP_TITLES` - default = "mommy/owner"
- `BOTTOMSPEAK_PETNAMES` - default = "sweetheart/sweetie/cutie/darling/honey"
- `BOTTOMSPEAK_PRAISE_TERMS` - default = "girl/pet"
- `BOTTOMSPEAK_PRINT_KEYWORDS` - default = "meow/mreow/mrrp/woof/wruff/yip"

All of these can be overriden when running the interpreter by simply passing them in on the commandline. Multiple potential values can be used by separating them with slashes (`/`) and then a random one is chosen from the pool each time, with the exception of `BOTTOMSPEAK_PRINT_KEYWORDS` where all options are always valid.

## REPL

BottomSpeak also features an interactive REPL so you can play around with the language with ease. It includes a few basic commands for interacting with the REPL and should be such sufficient for simpler programs.

## Installation

BottomSpeak is available through `cargo`, which you can get by installing `rustup` from the [official site](https://rustup.rs/) or your package manager of choice.
Simply run `cargo install bottomspeak` and you're done :3

## Notes

None of this project was made with AI and none of my projects ever will be. Fuck AI and fuck everything AI companies stand for. Celebrate human-made art in all forms and don't let anybody take away your ability to think for yourself 🧡.
