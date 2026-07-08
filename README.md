# BottomSpeak

BottomSpeak is the ultimate language for expressing yourself through programming. Expressions? Boring. Types? Who needs 'em?
Now you can program with the same kind of basic symbols you'd use when faced with even the slightest amount of dominance!

BottomSpeak is a stack-based language and as such has a very simple instruction set based around manipulating said stack.

## Language Syntax

BottomSpeak does not require a specific file extension, so feel free to use whatever you like, or none at all! All examples in the repo will use the `uwu` extension simply because why not?

### Keysmashes

These are how you push values to the stack. They can have a maximum length of 128 characters and consist purely of standard ASCII alphabet characters (a-zA-Z). The length of a keysmash is encoded as a byte value in the range 0-127 for lowercase keysmashes and 128-255 for uppercase ones.

For example, `asdfglaskdjh` has a length of 12 characters and thus is encoded as the value 11 in decimal because lowercase keysmashes start at 0, while `AKSDJHFPAOISDUJASDLKFJOI` has a length of 24 characters and thus is encoded as the value 151 because uppercase keysmashes start at 128.

Ordinarily, keysmashes must be separated by whitespace or a different case, otherwise they get parsed as a single keysmash, but it's also possible to use semicolons (`;`) to delimit them as they are also a common keysmash character e.g. `asdlkj dfkgjhasdo` == `asdlkj;dfkgjhasdo` and `sdlkjBAKLSDJ` == `sdlkj BAKLSDJ` == `sdlkj;BAKLSDJ`.

### Instructions

BottomSpeak features several instructions for simple stack manipulation:

- `>~<` - Pops the top value from the stack and discards it.
- `>w<` - Swaps the last two items on the stack.
- `:3` - Pops the last two values on the stack, adds them together and pushes the result back to the stack. Subtraction can also be achieved using overflow behaviour. Multiple values can be added together at once by simply repeating the `3` e.g. `:333` would add the top two stack values three times.
- `>//<` - Duplicates the value at the top of the stack. The value can be duplicated multiple times just like with `:3` by repeating the `//` e.g. `>//////<` would duplicate the value three times.
- `meow` - One of several [default keywords](#customisation) used for popping the top stack value and printing it as an ASCII character to stdout.
- `meow~` - Similar functionality to regular printing, but instead pops the last three stack values, pads them with an extra zero byte & constructs a unicode codepoint to be printed. For example, a stack with values `[1, 249, 122]` would be `[0x01, 0xf9, 0x7a]` in hexadecimal (or `[0x00, 0x01, 0xf9, 0x7a]` with padding), and thus is converted to the unicode codepoint `u1f97a` (🥺).

### Subroutines

Subroutines are a way to reuse code and as we all know, subs love getting used over and over again, so in order to define one, simply type what you'd normally type if someone told you that you were a good \<insert term here\>, like so:

```
alsdkfkl 🥺
```

Subs must of course finish at some point, so to do that simply use `>.<` at the end of the declaration. Finally, in order to get a subroutine to actually run, you can call it with a similar syntax to declarations:

```
alsdkfkl 👉👈
```

This will call the subroutine called `alsdkfkl` and begin doing as it's told like any good little sub should.

An important thing to note is that a subroutine call is actually a jump instruction, specifically jump-if-zero, which means the instruction will only execute if the stack is empty, or the top value is 0, which does allow for some branching in control flow.

### Comments

Comments are always inline and can be formed using the trans flag emoji '🏳️‍⚧️':

```
🏳️‍⚧️ Trans rights are human rights :3
```

## Error Reporting

BottomSpeak is a silly language but that doesn't mean it can't have good error reporting! The language shows you exactly where you went wrong and also gives some supportive words of encouragement, for example:

```
Mommy found some errors in your code but it's okay, honey, mommy believes in you <3

error[E0011]: oh sweetie, there aren't enough elements on the stack to add, could you try again for mommy?~
  ╭─ bleh.uwu:1:7
  │
1 │ haiii :3
  │       ^^
2 │
```

Now you can finally get some validation for all your hard work!

## Examples

Here is a simple "Hello, world!" program:

```
hawwo 🥺
  asdelkjla;afsdlkfjaksldkfjd >//////<
  :3333 >//< mreow
  dksfhapsduiofalsdkfjalsdkasdj :3 >//<
  aa :3 mreow >//<
  asdfkjhsl :3 >////< meow mrrp
  waow :3 yip
  asdkljfhasdklufhasdfasdhjflkjahld >////<
  wlsdkjfhaioua :3 meow mrrp >w< >//<
  fsdklfakl :3 >//<
  mkasdpfoasik :3 yip >//<
  bleh :3 mreow >//<
  asdlkvj :3 meow mrrp mreow
  um :3 mrrp >.<

hawwo 👉👈
```

As with any language based around printing ASCII via byte manipulation, there is no singular solution to a problem, so your "Hello, world!" could look very different to this one.
Feel free to submit a pull request with more complex/useful examples!

## Customisation

There are several behaviours which can be customised through environment variables.

- `DEFAULT_INTERP_TITLES` - default = "mommy/owner"
- `DEFAULT_PETNAMES` - default = "sweetheart/sweetie/cutie/darling/honey"
- `DEFAULT_PRAISE_TERMS` - default = "girl/pet"
- `DEFAULT_PRINT_KEYWORDS` - default = "meow/mreow/mrrp/woof/wruff/yip"

All of these can be overriden when running the interpreter by simply passing them in on the commandline. Multiple potential values can be used by separating them with slashes (`/`) and then a random one is chosen from the pool each time, with the exception of `DEFAULT_PRINT_KEYWORDS` where all options are always valid.

## REPL

BottomSpeak also features an interactive REPL so you can play around with the language with ease.
It includes a few basic commands which should be enough for simple programs.

## Installation

BottomSpeak is available through cargo, simply run `cargo install bottomspeak` and you're done :3

## Notes

None of this project was made with AI and none of my projects ever will be. Fuck AI and fuck everything AI companies stand for. Celebrate human-made art in all forms and don't let anybody take away your ability to think for yourself 🧡.
