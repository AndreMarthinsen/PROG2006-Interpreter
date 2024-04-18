![CI](https://img.shields.io/github/actions/workflow/status/VanWrinkle/PROG2006-Interpreter/rust.yml?branch=main)
# PROG2006 - Interpreter

## Description
This is one of two assignments in the course Advanced Programming at NTNU Gjøvik. The assignment was to implement an interpreter of a high level 
scripting language with syntax and functionality not too dissimilar from the functional programming language Haskell, along with a CLI tool allowing
interacting with the language through declaring variables, inputting lines of code and using commands to get information about types and function signatures.

## Use

### Demo
For a quick demo using `test_program.txt`, run the executable with `--src="test_program.txt"`.


### Program Arguments
- `-r` | `--repl-mode` - Runs the interpreter in repl-mode, where user input is continuously parsed and evaluated.
- `-h` | `--help` - prints information about interpreter usage to console.
- `-dbg` | `--debug` - prints tokens of a loaded file before starting executing it
- `-i` | `--indo` - Prints information about repl-mode usage
- `--src="<filename>"` - executes file. Assumes a plain text file with appropriate syntax.

### REPL
While in REPL mode, you can continuously put new values or expressions on the stack, run functions on stack values, or print information about what's on the stack.

#### Commands
- `:i` - Prints the type and value of the top element currently on the stack.
- `:h` - Prints list of commands and their use
- `:c` - Clears the stack
- `:dbg` - Toggles debug mode. While debugging, all contents of the stack will be printed.
- `:q` - Ends REPL mode and exits the application.






### Syntax
The language is stack based, meaning functions grab their arguments from a stack. This means that generally you will see postfix like notation.
The snippet below puts two numbers onto the stack, runs the function + on them and then puts the result back on the stack.
```
> 5 5 +
> 10
```






### Types

#### TypeClasses
The interpreter uses a type system based on Haskell where functions are defined on TypeClasses, classes that implement some trait compatible with the function. 


#### Strings

#### Numbers

#### Quotation

A `quotation` is a segment of code delimited by `{}`. Can be thought of as anonymous functions that evaluate to the contents when used with higher order functions.
Below, a quotation is put onto the stack, then executed with `exec`, leaving 10 on the stack.

```
bprog > { 5 5 + }
bprog > exec
bprog > :i
                Type:  Integer 
                Value: 10
```

#### 


### Native Functions

#### Higher Order Functions

- `map <func>` - takes a list and applies func to it.

```
bprog > [ 1 2 3 4 ] map { 1 + }
stack > [ 2 3 4 5 ]
```

- `each <func>` - Takes a list and applies func to each element. 

```
bprog > [ " count down! " 3 2 1 " liftoff! " ] each print
output> "count down!"
output> 3
output> 2 
output> 1
output> "liftoff!"
```

- `exec` - Executes a function from the top of the stack

- `foldl <(Any, Any ->Any)>` - Takes a list and a starting element with a binary function, reducing the list to a final value.

```
bprog > [ 1 2 3 ] 10 foldl { + } 
stack > 16
```

#### Math

`Func(Num, Num -> Num)`
- `+` - addition
- `-` - subtraction
- `/` - floating point division
- `*` - multiplication
- `div` - integer division

`Func(Integer, Integer -> Integer)`
- `%` - modulo

#### Logic

`Func(Boolean, Boolean -> Bool)`
- `&&` - Logical AND
- `||` - Logical OR

`Func(Num -> Num)`
- `not` - Negation. Works on numbers and booleans.

#### Comparisons

`Func(Ord, Ord -> Bool`
- `<`
- `>`
- `==`

#### Stack Operations
- `pop` - removes the top element of the stack
- `swap` - swaps the top elements of the stack
- `dup` - duplicates the top element of the stack


### Defining Functions




