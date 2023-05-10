# Project Documentation :: bprog interpreter

[TOC]



` # For later use

```css
<div style="max-width: 600px;">
This text will be limited to 600 pixels in width.
</div>
```

`



------

### Glossary

|             Term             | Explanation                                                  |
| :--------------------------: | ------------------------------------------------------------ |
|           **CLI**            | Text-based user interface used to run and interact with applications. |
|           **REPL**           | "Read-Eval-Print Loop", and environment where user input is read, evaluated and printed back out in a loop. |
|           **Type**           | A Type System is used to define a set of logically distinct entities capable of representing various categories of information we are familiar with from our day to day life, such as whole numbers and fractional numbers, known as Integer and Float in the `bprog` application. A function, or operation, is usually explicitly defined for a set of types. Another common type is String, representing a limited  body of text. |
|         **Variable**         | A generic term often used to describe entities of a Type that can change their value during runtime, but also constant entities that can never change their value once it has been set. In the `bprog` language there is no notion of constant values for variables. |
|        **Assignment**        | Assignment is the process of giving an instance of a Type a value, either for the first time, or reassigning the instance a new value. In `bprog` one can assign a new value to an instance by referring to its bound name. |
|   **Binding** / **Symbol**   | In programming languages it is often desired to associate an instance of a Type with a name for the purpose of referring back to it. A binding is the association of a name with such an instance. Such a binding could be the association of the name "frog_pond" to a String containing a brief description of a frog pond, but it could also be an instance of the Type yet to be assigned a value. In the `bprog` Type System, bindings are known as Symbols, and need not be assigned a specific Type or value. |
|          **Stack**           | A Last-In-First-Out (LIFO) data structure, where the last inserted entity is the first to be retrieved. The `bprog` application provides a runtime environment for a stack based programming language, where values are pushed onto a stack and operations/functions consume these values from the stack as they are executed. |
|  **Functions / Quotations**  | Functions are sections of code that have been given a name so one can repeatedly make use of it by referring to its name within a program. Functions in most languages can have clearly defined parameters, meaning variables they expect as input to work on. However, in  `bprog` such values are instead retrieved from a stack data structure by the function instead of being explicitly passed. In `bprog`, any block of code delimited by { and } can be considered a function, along with any predefined functions in the language. |
| **Parameters and Arguments** | A parameter refers to a variable defined as part of a function definition, and functions work on these variables during their life-time. An argument is the value of a parameter passed to such a functoin when it is used. In `bprog` such values are consumed from the top of the stack. |
|          **Return**          | Functions often return a new value as the result of its computations. Such a value is often referred to as its return value.  In the context of `bprog`, when we speak of a function and its return value, we mean the values it is expected to have added to the stack once done. |
|   **Function - Signature**   | The signature of a function defines the types of its input parameters as well as its return values, i.e. what types it is expected to return once it is done. An example would be `div`, which is a function that does integer division. Using the method used to describe function signatures within this documentation, it would look like this: `(Integer, Integer -> Integer)`. In some cases, a function may take variable types as a parameter. In those cases, such as getting the first element of a list, we use T as a placeholder.  `(List -> T)` . Here the signature indicates that a list is consumed and an entity T is put on top of the runtime stack. In other cases, T may be explicitly restricted to a sub-set of the types available in the `bprog` type system. |
|         **Literals**         | Where a binding can be used to represent a value as a name, literals are value representations written outright in the code, such as 1 used to represent the integer 1, and " duck pond " used to represent a String literal. |
|         **Integer**          | A type representing whole numbers, such as -1, 0, and 100.   |
|          **Float**           | A type representing numbers with a fractional part, such as 5.0, 3.141592, and -17.2412. In `bprog`, for a literal to be recognized as a Float, it must be trailed by at least one comma separated zero. |
|         **Boolean**          | A Type encoding a truth value. Booleans are either True or False. |
|          **String**          | A type representing a section of text. In `bprog` , string literals are delimited by the " sign. |
|                              |                                                              |
|                              |                                                              |
|                              |                                                              |
|                              |                                                              |



### Legend

|     `(* -> *)`     | Indicates the signature of a function, where * can be replaced with any number of various types. Left hand of the `->` represents what is expected to be popped off the stack, whereas the right hand side is what is expected to be put back onto the stack. |
| :----------------: | ------------------------------------------------------------ |
|        `T`         | T is a placeholder for any type. Unless specificed, T means a function can take any base primitive such as Integer, Float, Boolean and String. |
| `{quotation_name}` | A named quotation for referral within descriptive text.      |





------

## Requirement Specification

The following table outlines the functional requirements for this project. These requirements specify what the software should do to meet the needs and expectations of its users. The requirements are organized by ID, and each requirement is described in detail to ensure a clear  understanding of what the software is expected to accomplish. For project specific terminology, see the glossary section.

Each functional requirement is accompanied with an assumptions section, allowing developers to clarify any assumptions made in regards to the functional requirements.



**Priority**

To indicate the importance of the implimentation of each functional requirement, each identified requirement has been assigned an overarching priority, along with a priority for each of it's sub-requirements in instances where their priority deviates from that of the parent requirement. For clarification of each priority level, see the table below.

| Priority | Semantics                                                    |
| -------- | ------------------------------------------------------------ |
| Low      | Low priority, desirable if resources allow.                  |
| Medium   | Non-core functionality, but will impact the deliverable quality to a high degree. |
| High     | Core functionality that is expected in the final product.    |

> Note: For the sake of simplicity, I am setting priorities based on the minimal subset for grades D and C, setting these to High, and assigning Medium and Low to anything that goes beyond this. Setting priorities like this is usually a much more involved process, and I have no doubt that inclusion of a proper priority assessment with rationale behind each prioritization would be a great addition to the document, but time is a factor. 





#### F1: Basic Operation

|   ID   | Description                                                  | Priority | Dependencies |
| :----: | :----------------------------------------------------------- | :------: | ------------ |
| **F1** | **The software must function as a runtime interpreter for a stack based script language with postfix based syntax, where whitespace is used to delimit entities such as functions and primitives. ** | **High** |              |
|  F1.1  | As the language, and by extension its operations, is stack based, the runtime system has to support this by providing a stack that holds the values the program works on. |          |              |
|  F1.2  | The application must be capable of continuous CLI interaction, referred to as REPL mode. For every interaction, the application should print the contents of the stack after input has been processed and executed. |          |              |
|  F1.3  | The application must be capable of reading an entire program from a file, to be referred to as NORMAL mode. At the end of executing a whole program, the stack should contain a single value, which should be printed to the terminal window. |          |              |
|  F1.4  | The application should utilize a primitive scoping rule where all symbols are globally available. |          |              |
|  F1.5  | The runtime system must be capable of interpreting any syntactically correct input text as the appropriate types and structures defined in the language. |          |              |



**Assumptions:**

| ID   | Description | Rationale |
| ---- | ----------- | --------- |
|      |             |           |



#### F2: Data Types and Containers

|   ID   | Description                                                  | Priority | Dependencies |
| :----: | :----------------------------------------------------------- | :------: | ------------ |
| **F2** | **The application must be capable of parsing/handling a variety of data types and structures.** | **High** |              |
|  F2.1  | Must be able to represent and evaluate Symbol, Integer, Float, Bool, String and Lists. Lists may hold arbitrary types. |          |              |
|  F2.2  | Must be able to parse 'quotations'. Quotations are to be understood as a block of code delimited by { } symbols. Must be able to behave like any other section of code. More generally, one should be able to enclose the entire program in { } and execute it, and have it behave the same way as if it was not enclosed in a quotations at all. Note that explicitly executing it through a command/operation is an important distinction. |          |              |
|  F2.3  | Must be able to parse reserved key-words like `+` and `map`. |          |              |

**Assumptions:**

| ID   | Description | Rationale |
| ---- | ----------- | --------- |
|      |             |           |



#### F3: Type Coercion

|   ID   | Description                                                  |  Priority  | Dependencies |
| :----: | :----------------------------------------------------------- | :--------: | ------------ |
| **F3** | **The application should support type coercion. i.e. implicit casting from one type to another where appropriate.** | **Medium** |              |
|  F3.1  | Should support coercion between Float, Integer and Boolean. If an operation outputs a flexible type, it should output the type that prevents any data loss. Examples: Int + Float = Float, Bool + Any = Any, Float + Any = Any. |            |              |
|  F3.2  | In the case of operations expecting a certain type, if the operands can be cast into the expected type, this should be done implicitly. Example: 20. |            |              |

**Assumptions:**

| ID   | Description | Rationale |
| ---- | ----------- | --------- |
|      |             |           |



#### F4: Visual Representation

|   ID   | Description                                                  | Priority | Dependencies |
| :----: | :----------------------------------------------------------- | :------: | ------------ |
| **F4** | **The application must be capable of providing a visual representation of the contents of the stack.** | **High** |              |
|  F4.1  | Lists should print on a compact form; Example: "[1,2,3,[]]". |          |              |
|  F4.2  | Floats should always print with at least one precsision point, such as "5.0", and not "5". |          |              |
|  F4.3  | Symbols should  print out as their name when present on the stack. |          |              |
|  F4.4  | Reserved key-words should print out as their name when found inside quotations. |          |              |
|  F4.5  | Quotations should print on the form "{ 1 2 + 3 div }".       |          |              |

**Assumptions:**

| ID   | Description | Rationale |
| ---- | ----------- | --------- |
|      |             |           |



#### F5: Symbol Dictionary

|   ID   | Description                                                  | Priority | Dependencies |
| :----: | :----------------------------------------------------------- | :------: | ------------ |
| **F5** | **The application must be capable of mapping from pre-defined and user defined Symbols to functions and values in a Symbols dictionary.** | **High** |              |
|  F5.1  | The application must be capable of checking if a Symbol conflicts with a reserved key-word such as `append`. |          |              |
|  F5.2  | The application should allow reassignment of non-reserved Symbols to new values. |          |              |
|  F5.3  | Symbols that are yet to be bound to anything evaluate must evaluate to themselves, whereas bound symbols evaluate to a value. |          |              |
|  F5.4  | The application should in addition to the base set of functionality be capable of importing any other library functionality, read and defined in a file named `prelude.bprog` that is read before running REPL or NORMAL mode. |  Medium  |              |

**Assumptions:**

| ID   | Description | Rationale |
| ---- | ----------- | --------- |
|      |             |           |



#### F6: IO Operations

|   ID   | Description                                                  | Priority | Dependencies |
| :----: | :----------------------------------------------------------- | :------: | ------------ |
| **F6** | **The application must provide support for the following basic IO operations.** |   High   |              |
|  F6.1  | `print` - prints the top element of the stack.               |          |              |
|  F6.2  | `read` - reads a string from STDIN and places it on top of the stack. |          |              |

**Assumptions:**

| ID   | Description | Rationale |
| ---- | ----------- | --------- |
|      |             |           |



#### F7: Stack Operations

|   ID   | Description                                                  | Priority | Dependencies |
| :----: | :----------------------------------------------------------- | :------: | ------------ |
| **F7** | **The application must provide support for the following stack operations.** | **High** |              |
|  F7.1  | `dup` - duplicates the top stack element and puts it on top of the stack. |          |              |
|  F7.2  | `swap` - swaps the top two stack elements.                   |          |              |
|  F7.3  | `pop` - removes the top element of the stack.                |          |              |

**Assumptions:**

| ID   | Description | Rationale |
| ---- | ----------- | --------- |
|      |             |           |



#### F8: Runtime Parsing

|   ID   | Description                                                  | Priority | Dependencies |
| :----: | :----------------------------------------------------------- | :------: | ------------ |
| **F8** | **The application must support a set of functions for parsing text to in program values. All of these should generate an error on failure to parse the string into an expected type. In the event of the top element not being a string, an error should also be generated.** | **High** |              |
|  F8.1  | `parseInteger`- parses the top stack element into an Integer. |          |              |
|  F8.2  | `parseFloat` - parses the top stack element into a Float.    |          |              |
|  F8.3  | `words` - parses the top element into a list of tokens.      |  Medium  |              |

**Assumptions:**

| ID   | Description | Rationale |
| ---- | ----------- | --------- |
|      |             |           |



#### F9: Arithmetic

|   ID    | Description                                                  | Priority | Dependencies |
| :-----: | :----------------------------------------------------------- | :------: | ------------ |
| **F10** | **The application must support basic arithmetic operations between numerical values, including but not restricted to `+`, `-`, `*`, `/` and `div`, where div is restricted to integers.** | **High** |              |

**Assumptions:**

| ID   | Description | Rationale |
| ---- | ----------- | --------- |
|      |             |           |



#### F10: Comparisons

|   ID    | Description                                                  | Priority | Dependencies |
| :-----: | :----------------------------------------------------------- | :------: | ------------ |
| **F10** | **The applications must support comparisons between numeric values, Integer and Float, through the use of the `<`, `>` and `==` operations.** | **High** |              |

**Assumptions:**

| ID   | Description | Rationale |
| ---- | ----------- | --------- |
|      |             |           |



#### F11: Logical Operations

|   ID    | Description                                                  | Priority | Dependencies |
| :-----: | :----------------------------------------------------------- | :------: | ------------ |
| **F11** | **The application must support the basic logical operations `&&`, `||` and `not`. The base implementation expects boolean operands.** | **High** |              |
|  F11.1  | `not` may also function as generalized negation of numeric values in addition to booleans. |          |              |

**Assumptions:**

| ID   | Description | Rationale |
| ---- | ----------- | --------- |
|      |             |           |



#### F12: List Operations

|   ID    | Description                                                  | Priority | Dependencies |
| :-----: | :----------------------------------------------------------- | :------: | ------------ |
| **F12** | **The application must support the operations below for Lists. Where a quotation is shown, it is taken from the program representation (i.e. list or tree of values), not the stack.** | **High** |              |
|  F12.1  | `head` - `(List -> T)`: Takes the first element of a list from the top of the stack, placing it on top of the stack. |          |              |
|  F12.2  | `tail`- `(List -> List)`: The inverse of `head`, instead placing the remainder of the list on top of the stack after the first element has been removed. |          |              |
|  F12.3  | `empty` - `(List -> Boolean)`: Checks whether a list is empty or not, placing the resulting `True` or `False` primitives atop the stack. |          |              |
|  F12.4  | `length` -  `(List -> Integer)`: Checks the length of a list atop the stack and places its length as an `Integer` on top of the stack. |          |              |
|  F12.5  | `cons` - `(T, List -> List )`: Pops a value and a list off of the stack and creates a new list with the value as the new head, placing it back onto the stack. |          |              |
|  F12.6  | `append`- `(List, List -> List)`: Pops two lists off the stack and concatenates them into a single list, placing it back onto the stack. |          |              |

**Assumptions:**

| ID   | Description | Rationale |
| ---- | ----------- | --------- |
|      |             |           |



#### F13: Higher Order Functions 

|   ID    | Description                                                  | Priority | Dependencies |
| :-----: | :----------------------------------------------------------- | :------: | ------------ |
| **F13** | **The application should support a basic set of higher order functions on lists** | **High** |              |
|  F13.1  | `each` `{apply}` - `(List -> Void)`: Performs the `apply` quotation  on each element in the list. Does not inherently result in a new value being put onto the stack. |          |              |
|  F13.2  | `map` `{alter}`-  `(List -> List)`: Applies the closure `alter` to each element in the list before putting the list back onto the stack in place of the old one. |          |              |
|  F13.3  | `foldl` `{accumulate}` `(T, List -> T)`: Expects a starting value `T` and list from the stack, using the `accumulate` quotation to iteratively modify the initial`T` value before putting the final result onto the stack. |          |              |

**Assumptions:**

| ID   | Description | Rationale |
| ---- | ----------- | --------- |
|      |             |           |



#### F14: Control Flow

|   ID    | Description                                                  | Priority | Dependencies |
| :-----: | :----------------------------------------------------------- | :------: | ------------ |
| **F14** | **The application must support basic control flow with the following operations:** | **High** |              |
|  F14.1  | `if` `{then}` `{else}` : Checks the boolean value of the top of the stack and executes `then` if it evaluates to `True`, executing the `else` quotation otherwise. |          |              |
|  F14.2  | `loop` `{break}` `{do}`: Repeatedly executes the `do` quotation until `break` evaluates to `True`. |  Medium  |              |
|  F14.3  | `times` `{do}` `(Integer -> *)`: Performs the `do` quotation x amount of times depending on the input Integer. |          |              |

**Assumptions:**

| ID   | Description | Rationale |
| ---- | ----------- | --------- |
|      |             |           |



#### F15: Quotation Operations

|   ID    | Description                                                  | Priority | Dependencies |
| :-----: | :----------------------------------------------------------- | :------: | ------------ |
| **F15** | **The program must support the following directives:**       | **High** |              |
|  F15.1  | `exec` - `(Quotations -> *)`: Takes a quotation from the top of the stack and executes it once. |          |              |

**Assumptions:**

| ID   | Description | Rationale |
| ---- | ----------- | --------- |
|      |             |           |



#### F16: Symbol Dictionary and Symbol->Value Mapping

|   ID    | Description                                                  |  Priority  | Dependencies |
| :-----: | :----------------------------------------------------------- | :--------: | ------------ |
| **F16** | **The program must allow assigning values to a Symbol. The binding operations do not return a value to the stack, but should instead result in a mapping independent of the stack.** | **Medium** |              |
|  F16.1  | `:=` - `(Symbol, T -> Void)`: Pops a symbol and a value of any kind that is **not** a Symbol off of the stack, assigning the value to the symbol. |            |              |
|  F16.2  | `fun`- `(Symbol, Quotation -> Void)`: Pops a Symbol and a quotation off the stack and maps from the symbol to the quotation. `fun` is a more restrictive version of `:=`. |            |              |
|  F16.3  | `'` `a_symbol` : ' is an operator that tells the program to put the raw symbol onto the stack, rather than the value it evaluates to. As symbols normally evaluate to themselves if bound, this operator can be used to rebind already bound symbols. |            |              |
|  F16.4  | `eval` `(Symbol -> T)` : Takes a Symbol from the top of the stack and replaces it with the value it maps to. An unbound Symbol evaluates to itself. |            |              |

**Assumptions:**

| ID   | Description | Rationale |
| ---- | ----------- | --------- |
|      |             |           |









------

## Non-Functional Requirements

Below you can find sections detailing requirements in terms of implementation with respect to the functional requirements in the previous section. 

Non-functional requirements ID's are formatted as {Functional Requirement ID}-{metric-nr}. Example: F-1.1-PE.1 in instances where they are tied directly to a functional requirement section, otherwise they are simply identified as {Gen-metric.nr}, such as Gen-MA.1. 

> **Legend:**
>
> **Performance    : PE**
>
> **Usability            : US**
>
> **Security              : SE**
>
> **Stability		 	 : ST**
>
> **Scalability           : SC**
>
> **Maintainability : MA**



### Performance

| ID       | Requirements                                                 |
| -------- | ------------------------------------------------------------ |
| **F-1**  | **Persistence of Game State**                                |
| F-1-PE   | The file loading time should be minimized to avoid any noticeable delay or adverse effects on the game experience. |
| **F-2**  | **Visualization and UI**                                     |
| F-2-PE   | The graphical representation of the game should have a high frame rate  to ensure that the user experience does not feel sluggish. |
| **F-3**  | **AI Opponent**                                              |
| F-3-PE.1 | The AI algorithm should run in a separate thread or be allotted a  specific work-time per frame to ensure that it does not impact the  responsiveness of the UI. |
| F-3-PE.2 | The board representation should have a small memory footprint and  support fast computation of heuristics and valid board states to ensure  that the AI opponent can run efficiently. |
| F-3-PE.3 | The weighting of heuristics should be easily modifiable to allow for rapid tweaking of AI behavior. |
| F-3-PE.4 | The software should be designed to handle larger board sizes with  minimal impact on performance to ensure that the user experience is  consistent regardless of the game board's size. |
|          |                                                              |



### Usability

| ID       | Requirements                                                 |
| -------- | ------------------------------------------------------------ |
| **F-1**  | **Persistence of Game State**                                |
| F-1-US   | The software must provide clear instructions on how to load and save game states using the .SGF file format, and any relevant limitations or restrictions. |
| **F-2**  | **Visualization and UI**                                     |
| F-2-US.1 | The interaction with the game state should be intuitive and easy to  understand, allowing the user to easily make moves and understand the  state of the board without confusion or difficulty. |
| F-2-US.2 | The graphical representation of the board state should be visually  appealing, easy to distinguish, and consistent throughout the software,  allowing the user to quickly and easily understand the state of the  board. |
| F-2-US.3 | The interface should provide clear and concise visual cues or feedback  to the user when performing an action, such as highlighting legal moves  or indicating captured stones, ensuring that the user can easily  understand the consequences of their actions. |
| **F-3**  | **AI Opponent**                                              |
| F-3-US.1 | The software must provide clear instructions on how to access and use the AI opponent, including any difficulty levels or settings that can be adjusted if implemented in the software. |
| F-3-US.2 | The software should provide clear and concise instructions on how to access and use the AI heuristics to get hints when prompted if the feature is implemented. |
| **F-4**  | **Rules**                                                    |
| F-4-US.1 | The software must provide clear and concise instructions on the rules of the game, including how to win, how to count score, and any special rules or exceptions. |
| F-4-US.2 | The software should provide clear visual cues or feedback when a capture or illegal move occurs, including highlighting captured stones or indicating illegal moves. |
| F-4-US-3 | The software should provide a way for the player to easily request or view the current score of the game. |
|          |                                                              |



### Security

| ID       | Requirement                                                  |
| -------- | ------------------------------------------------------------ |
| Gen-SE.1 | The software must not provide unauthorized access to system resources. |
| Gen-SE.2 | The software must comply with privacy regulations and not collect or  transmit any personally identifiable information or sensitive data  without explicit user consent. |
| Gen-SE.3 | The software must not be capable of executing any input text as code or otherwise cause harm through the parsing of user input or files. |
|          |                                                              |

> Note: As the software is required to operate with the SGF file format, security measures intended to prevent tampering with the game state through the SGF file directly is not considered desirable for this project. 



### Stability

| ID       | Requirement                                                  |
| -------- | ------------------------------------------------------------ |
| F-1-ST   | The software must be stable and reliable when loading and storing game  state data in the .SGF format to prevent any loss or corruption of game  data. |
| F-2-ST   | The software must be able to handle all possible user input during its runtime without causing crashes or unexpected game behavior. |
| Gen-ST.1 | The software must not be capable of causing system instability, such as  taking ownership of system resources to an extent where the user or OS cannot  shut down the application properly. |



### Scalability 

As there are no scalability issues associated with user count or other similar issues, this field is left blank. Scaling of AI workload when upscaling the game size is addressed under the Performance section.



### Maintainability

| ID       | Requirement                                                  |
| -------- | ------------------------------------------------------------ |
| Gen-MA.1 | The code should be easy to read, understand, and modify. The naming of variables, functions, and classes should be clear and descriptive. |
| Gen-MA.2 | The code should be structured in a modular way, with separate functions or classes for different parts of the functionality. |
| Gen-MA.3 | The code should be designed in a way that makes it easy to extend and add new features. |
| Gen-MA.4 | The code should be testable, with unit tests for individual functions and integration tests for the entire system. |
| Gen-MA.5 | The code should have clear error handling mechanisms in place, with error messages that are informative and actionable. |
| Gen-MA.6 | The code should have well-documented functions, classes, and modules, with clear explanations of their purpose and usage. |
| Gen-MA.7 | The code should be designed to minimize the accumulation of technical debt, with an eye towards long-term maintainability. |
| Gen-MA.8 | The code should be consistent in terms of naming conventions, code structure, and coding style. |



### Compatibility

| ID       | Requirement                                                  |
| -------- | ------------------------------------------------------------ |
| Gen-CO.1 | The software should be compatible with a variety of common screen sizes and resolutions. |
| Gen-CO.2 | Should be compatible with most modern operating systems such as Linux, MacOS and Windows. |
|          |                                                              |







## Implementation

WIP



## Project Conventions

### Constants

- No 'magic values' in the code base, meaning no un-explained hard coded literals.

### Commenting:

- Data types should have at least a minimal comment explaining their intended use and function.
- Never leave commenting for later. Comment right away, preferably with doctest commenting included.
- All code should be self documenting. Function and binding names should provide enough information for trivial logic while commenting is reserved for non-trivial logic.
- If you are unsure if a segment of code is trivial, err on the side of caution and add a brief explanation.
- TODO: No TODO: comments should be merged into the main branch. TODO sections are expected to be handled prior to merge. 

#### Naming:

- Function and binding names must be in camelCase.
- Avoid single letter names unless it is a variable with a short lifespan with an obvious purpose. Err on the side of caution and add choose a more descriptive name if you are unsure.
- Avoid 'noise' in names, such as customerList where list can readily be inferred from the function type signature.
- Avoid abbreviations or acronyms unless the scope is limited and the purpose of the binding is obvious from the context.

### Git-usage

#### Issues

- Issues should be linked to a Milestone.
- Issues should have a description with enough detail that there is no ambiguity as to what must be implemented for the issue to be considered ready for closure.

#### Branching

- Issues should have associated branches.
- The main branch should be protected. All work is to be pushed to separate branches before being merged into main.

#### Commits

- Commit messages should be linked to issues through use of keywords such as 'Closes #x', 'Relates to #x", and other relevant keywords in use by gitlab.
- Commit scope: The scope of a commit should be atomic, i.e. it should address one problem / task and have a clearly defined scope.
- If a commit does a multitude of changes across the codebase in multiple files the commit message should give an exhaustive detailing of what problem is being addressed, and all changes should work towards one over-arching goal.

#### Merging

- Merging into main should not introduce any new bugs or cause the application fail to run.

#### Cleanliness

- The repository should only contain relevant, portable code and documentation. No binaries or IDE specific files should be pushed to the repository.

### Testing

- Testing should be done with doctest in the cases of trivial functions, but for more complex functions a more thorough test suite should be used.









## Assessment Specification

Below is a schema intended for the use of assessing the degree of success of this project.



**Scoring:**

The criteria are judged by a scoring on a 1-5 scale where the score reflects the satisfaction level with respect to the criteria.

| Score | Signifies                                                    |
| ----- | ------------------------------------------------------------ |
| 0     | Not addressed to any degree                                  |
| 1     | Significantly below expectations                             |
| 2     | Below expectations, with several errors or omissions         |
| 3     | Meets expectations, with some errors or omissions            |
| 4     | Exceeds expectations, with very few errors or omissions      |
| 5     | Outstanding, with no errors or omissions, excellent attention to detail |

------



### Functional and Non-Functional Requirements

**Clarity**

|      | Criteria                                                     | Score |
| ---- | ------------------------------------------------------------ | ----- |
| 1    | Does the project distinguish between functional requirements received from the stake-holders and what is assumed by  the developers? |       |
| 2    | Does the documentation make an effort to reduce or nullify ambiguity through the use of in-line definitions, legends or glossaries where appropriate? |       |
| 3    | Is the language used in the requirements accessible to all potential stake-holders? |       |
| 4    | Are the functional requirements consistent with the project scope, goals, and objectives? |       |
| 5    | Are the non-functional requirements linked to functional requirements where appropriate, and if so, is it made explicit which ones? |       |
| 6    | Are the non-functional requirements organized into respective categories such as Performance, Usability, Security and others? |       |
| 7    | Is it made clear what functional requirements are implemented and to what degree? |       |
| 8    | Is it made clear to what degree the implementation satisfies the non-functional requirements? |       |
| 9    | Is it clear why a particular implementation has been chosen? |       |
| 10   | Does the documentation clearly convey what features are core features? |       |
| 11   | To what degree are the functional and non-functional requirements testable and verifiable? |       |
|      | **Total** out of 55                                          |       |

**Comments**

|      |
| ---- |



**Completeness**

|      | Criteria                                                     | Score |
| ---- | ------------------------------------------------------------ | ----- |
| 1    | To what degree have critical functional requirements been addressed? (e.g. those that are essential to the software's basic functionality) |       |
| 2    | To what degree have non-critical functional requirements been addressed? (e.g. those that enhance the software's usability or performance) |       |
| 3    | To what degree have non-functional requirements been addressed? (e.g. those related to security, scalability, etc.) |       |
| 4    | Is the project free of any known gaps in the software's functional requirements? |       |
| 5    | Is the project free of any known gaps in the software's non-functional requirements? |       |
|      |                                                              |       |
|      | **Total** out of 25                                          |       |

**Comments**

|      |
| ---- |



**Accuracy**

|      | Criteria                                                     | Score |
| ---- | ------------------------------------------------------------ | ----- |
| 1    | To what degree does the project utilize testing to ensure that the deliverable meets the requirements and needs of the stake-holder? |       |
| 2    | Are there specific test cases for each requirement that can be traced  back to the original functional or non-functional requirement? |       |
| 3    | Are the requirements validated by the stake-holders to ensure that they accurately reflect their needs and expectations? |       |
| 4    | Has the developer provided evidence of successful testing and validation of requirements? |       |
|      | **Total** out of 20                                          |       |

**Comments**

|      |
| ---- |



### Project Organization

|      | Criteria                                                     | Score |
| ---- | ------------------------------------------------------------ | ----- |
| 1    | How well does the project define a set of conventions for the code-base, such as variable names, constants, commenting, separation of code, modules and other relevant aspects? |       |
| 2    | How well does the project define a set of conventions for the use of git, such as commit messages, use of issue tracker, linking of commits to issues and usage of milestones? |       |
| 3    | How well does the project maintain repository cleanliness and overall organization? |       |
| 4    | How well does the project organize its documentation, either through repository directories or use of wiki? |       |
| 5    | What is the quality of the main README.md file? Does it include installation instructions, information about project goal? |       |
|      | **Total** out of 25                                          |       |

**Comments**

|      |
| ---- |



### Code Quality

|      | Criteria                                                     | Score |
| ---- | ------------------------------------------------------------ | ----- |
| 1    | How well is the behavior of functions commented throughout the code-base? |       |
| 2    | How well is non-trivial logic commented in function bodies?  |       |
| 3    | How consistent is commenting throughout the code base?       |       |
| 4    | How well does the code-base adhere to the project conventions? |       |
| 5    | How well does the code-base maintain modularity?             |       |
| 6    | How well does the code-base adhere to SOLID principles of Single Responsibility, Open/Closed, Liskov Substitution, Interface Segregation, Dependency Inversion? |       |
| 7    | How well is the code-base tested, including coverage and effectiveness of test cases? |       |
|      | **Total **out of 35                                          |       |

**Comments**

|      |
| ---- |



### Deliverable Quality

|      | Criteria                                                     | Score |
| ---- | ------------------------------------------------------------ | ----- |
| 1    | How intuitive and easy to learn is the user interface?       |       |
| 2    | How consistent is the user interface across the application? |       |
| 3    | How responsive is the user interface to user actions?        |       |
| 4    | How well does the application handle errors and unexpected user input? |       |
| 5    | How well does the application handle different screen sizes and resolutions? |       |
| 6    | Does the application provide appropriate feedback to user actions, such as loading indicators and success/error messages? |       |
| 7    | How well does the application perform in terms of frame rate (if applicable) and response time when under heavy load? |       |
|      | **Total** out of 35                                          |       |

**Comments**

|      |
| ---- |



### Total Score

|                          | Score | out of  |
| ------------------------ | ----- | ------- |
| **Requirements**         |       |         |
| Clarity                  |       | 55      |
| Completeness             |       | 25      |
| Accuracy                 |       | 20      |
| **Project Organization** |       | 25      |
| **Code Quality**         |       | 35      |
| **Deliverable Quality**  |       | 35      |
| **Total**                |       | **195** |





## Self-Assessment

Below is a schema intended for the use of assessing the degree of success of this project.



**Scoring:**

The criteria are judged by a scoring on a 1-5 scale where the score reflects the satisfaction level with respect to the criteria.

| Score | Signifies                                                    |
| ----- | ------------------------------------------------------------ |
| 0     | Not addressed to any degree                                  |
| 1     | Significantly below expectations                             |
| 2     | Below expectations, with several errors or omissions         |
| 3     | Meets expectations, with some errors or omissions            |
| 4     | Exceeds expectations, with very few errors or omissions      |
| 5     | Outstanding, with no errors or omissions, excellent attention to detail |

------



### Functional and Non-Functional Requirements

**Clarity**

|      | Criteria                                                     | Score |
| ---- | ------------------------------------------------------------ | ----- |
| 1    | Does the project distinguish between functional requirements received from the stake-holders and what is assumed by  the developers? | 4     |
| 2    | Does the documentation make an effort to reduce or nullify ambiguity through the use of in-line definitions, legends or glossaries where appropriate? | 3     |
| 3    | Is the language used in the requirements accessible to all potential stake-holders? | 3     |
| 4    | Are the functional requirements consistent with the project scope, goals, and objectives? | 4     |
| 5    | Are the non-functional requirements linked to functional requirements where appropriate, and if so, is it made explicit which ones? | 5     |
| 6    | Are the non-functional requirements organized into respective categories such as Performance, Usability, Security and others? | 5     |
| 7    | Is it made clear what functional requirements are implemented and to what degree? | 0     |
| 8    | Is it made clear to what degree the implementation satisfies the non-functional requirements? | 0     |
| 9    | Is it clear why a particular implementation has been chosen? | 0     |
| 10   | Does the documentation clearly convey what features are core features? | 2     |
| 11   | To what degree are the functional and non-functional requirements testable and verifiable? | 3     |
|      | **Total** out of 55                                          | 29    |

**Comments**

|                                                              |
| ------------------------------------------------------------ |
| 7, 8, 9. Ran out of time in regards to doing a proper implementation section detailing chosen implementations and what has been implemented. |
|                                                              |



**Completeness**

|      | Criteria                                                     | Score |
| ---- | ------------------------------------------------------------ | ----- |
| 1    | To what degree have critical functional requirements been addressed? (e.g. those that are essential to the software's basic functionality) | 1     |
| 2    | To what degree have non-critical functional requirements been addressed? (e.g. those that enhance the software's usability or performance) | 2     |
| 3    | To what degree have non-functional requirements been addressed? (e.g. those related to security, scalability, etc.) | 2     |
| 4    | Is the project free of any known gaps in the software's functional requirements? | 2     |
| 5    | Is the project free of any known gaps in the software's non-functional requirements? | 2     |
|      |                                                              |       |
|      | **Total** out of 25                                          | 9     |

**Comments**

|                                                              |
| ------------------------------------------------------------ |
| 1. Some rules are in place, but actual fail safe parsing of SGF is not in place, neither does the program currently load it from an actual file, but rather from a hard coded string for demonstration purposes. |
| 2. Window resizes correctly. Spent far too long on that little detail due to resizing a window through code causes a new resize event which complicated things quite a bit. |
| 3. Computations are mostly event driven, so I guess you could say it performs alright in that regard, but there's not a whole lot to actually compute at the moment since there is no AI implemented. |



**Accuracy**

|      | Criteria                                                     | Score |
| ---- | ------------------------------------------------------------ | ----- |
| 1    | To what degree does the project utilize testing to ensure that the deliverable meets the requirements and needs of the stake-holder? | 1     |
| 2    | Are there specific test cases for each requirement that can be traced  back to the original functional or non-functional requirement? | 1     |
| 3    | Are the requirements validated by the stake-holders to ensure that they accurately reflect their needs and expectations? | 0     |
| 4    | Has the developer provided evidence of successful testing and validation of requirements? | 0     |
|      | **Total** out of 20                                          | 2     |

**Comments**

|                                                              |
| ------------------------------------------------------------ |
| 1. Some doctesting in the code-base, but no real testing to talk of. |



### Project Organization

|      | Criteria                                                     | Score |
| ---- | ------------------------------------------------------------ | ----- |
| 1    | How well does the project define a set of conventions for the code-base, such as variable names, constants, commenting, separation of code, modules and other relevant aspects? | 3     |
| 2    | How well does the project define a set of conventions for the use of git, such as commit messages, use of issue tracker, linking of commits to issues and usage of milestones? | 2     |
| 3    | How well does the project maintain repository cleanliness and overall organization? | 3     |
| 4    | How well does the project organize its documentation, either through repository directories or use of wiki? | 3     |
| 5    | What is the quality of the main README.md file? Does it include installation instructions, information about project goal? | 0     |
|      | **Total** out of 25                                          | 11    |

**Comments**

|      |
| ---- |
|      |



### Code Quality

|      | Criteria                                                     | Score |
| ---- | ------------------------------------------------------------ | ----- |
| 1    | How well is the behavior of functions commented throughout the code-base? | 4     |
| 2    | How well is non-trivial logic commented in function bodies?  | 1     |
| 3    | How consistent is commenting throughout the code base?       | 2     |
| 4    | How well does the code-base adhere to the project conventions? | 0     |
| 5    | How well does the code-base maintain modularity?             | 2     |
| 6    | How well does the code-base adhere to SOLID principles of Single Responsibility, Open/Closed, Liskov Substitution, Interface Segregation, Dependency Inversion? | 2     |
| 7    | How well is the code-base tested, including coverage and effectiveness of test cases? | 2     |
|      | **Total **out of 35                                          | 13    |

**Comments**

|                                                              |
| ------------------------------------------------------------ |
| While I've commented most if not all functions describing what they do, the descriptions are not detailed enough. There are a lot of non trivial functions involving bitwise operations that some may not be familiar with, as well as some non-obvious behaviour in regards to translation of indices that warrant more detailed comments inside the function bodies. |
| There are no major bugs that can be triggered by game play, but loading SGF files containing ranges such as ;A[aa:ff] causes the program to crash as I've not implemented logic for handling it. There is some testing in place for the majority of the functions in GoBoard.hs, but none in Lib or Common. A lot of functions are partial to avoid overhead in the form of checks, intended for use with the AI, but I never got that far. |





### Deliverable Quality

|      | Criteria                                                     | Score |
| ---- | ------------------------------------------------------------ | ----- |
| 1    | How intuitive and easy to learn is the user interface?       | 3     |
| 2    | How consistent is the user interface across the application? | 3     |
| 3    | How responsive is the user interface to user actions?        | 4     |
| 4    | How well does the application handle errors and unexpected user input? | 4     |
| 5    | How well does the application handle different screen sizes and resolutions? | 4     |
| 6    | Does the application provide appropriate feedback to user actions, such as loading indicators and success/error messages? | 3     |
| 7    | How well does the application perform in terms of frame rate (if applicable) and response time when under heavy load? | 3     |
|      | **Total** out of 35                                          | 24    |

**Comments**

|                                                              |
| ------------------------------------------------------------ |
| The features that are implemented at least work as intended, aside SGF loading, but with how little is implemented that's not saying much. |



### Total Score

|                          | Score | out of  |
| ------------------------ | ----- | ------- |
| **Requirements**         |       |         |
| Clarity                  | 29    | 55      |
| Completeness             | 9     | 25      |
| Accuracy                 | 2     | 20      |
| **Project Organization** | 6     | 25      |
| **Code Quality**         | 13    | 35      |
| **Deliverable Quality**  | 24    | 35      |
| **Total**                | 88    | **195** |



### Self Reflection

A thing that has become very obvious to me at this point is that I've started in the wrong end. I worked on the application for a long time without there being a single sliver of documentation written, no specifications, no functional requirements, no nothing.

Had I started with the documentation, as boring and grueling as that sounds, I would have had a much deeper understanding of the work-load and the order in which things should have been done. Additionally, it could have saved me from a lot of painful refactoring early on in the assignment. When you are drowning in school-work it is tempting to simply start coding, as that's the fun part, but without a well structured plan to follow you risk taking a wrong turn a few times too many, putting your efforts into the wrong aspects of the application. 

Case in point, my application can be resized perfectly fine. You can full-screen it, and it resizes just fine then too. While a good feature to have, it is nowhere near as important as having an actually functioning rule set, some form of basic computer opponent and functioning persistence of game state. My focus was not to deliver a functional piece of software to my 'customer', as it should have been, but instead to implement bit-board representation of the game state to support faster computation so that I could implement some of the algorithms detailed in http://erikvanderwerf.tengen.nl/pubdown/thesis_erikvanderwerf.pdf, but those heuristics are overkill for the scope of the assignment. That is a weakness of mine I need to learn to temper; focusing too hard on things that seem interesting or fun that do not line up with the needs or priorities of the stake-holder, or assignments in general.

A majority of missing functional requirements are trivial to implement at this point, and should be easy to put in place for the portfolio, but I chose to focus on the documentation towards the end to have a solid foundation to work on for assignment 2 and the second iteration of assignment 1. 













