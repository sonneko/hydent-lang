## Keywords

| Keyword | Description | Brief Description | Examples |
| :--- | :--- | :--- | :--- |
| **`import`** | Declares dependencies by bringing definitions from other modules into scope. | **Bring code into scope** | `import { Math } from "lib";`, `import * as io from "std::io";` |
| **`static`** | Used to declare variables or members that belong to the type itself, not an instance. | **Type-level variable** | `pub static PI = 3.14;`, `static mut COUNTER = 0;` |
| **`class`** | Declares a user-defined blueprint for creating objects, often involving state and behavior. | **Define object blueprint** | `class User {}`, `class Car<T> implements Vehicle {}` |
| **`enum`** | Declares a type that can be one of a fixed set of named variants. | **Define fixed variants** | `enum Result { Ok, Err(String) }`, `enum Color { Red, Green, Blue }` |
| **`struct`** | Defines a type that encapsulates a collection of named fields (data structure). | **Aggregate named fields** | `struct Point { x: Int, y: Int }`, `struct (Int, String);` |
| **`fn`** | Used to declare a function or method. | **Declare a function** | `fn add(a: Int, b: Int): Int {}`, `pub async fn load() {}` |
| **`protocol`** | Declares a blueprint of methods or properties that a `class` or `enum` can conform to. | **Define type contract** | `protocol Hashable {}`, `protocol Readable: Closable {}` |
| **`module`** | Organizes code into distinct logical units. | **Code organization unit** | `module network {}`, `pub module core {}` |
| **`type`** | Creates an alias for an existing type. | **Create type alias** | `type ID = Usize;`, `pub type List<T> = Array<T>;` |
| **`pub`** | Access modifier indicating that an item is publicly visible. | **Public visibility modifier** | `pub class Data {}`, `pub fn get_value() {}` |
| **`extern`** | Declares a function defined outside the current language/runtime, often in C or assembly. | **External linkage declaration** | `extern fn malloc(size: Usize): Any;` |
| **`async`** | Marks a function as asynchronous, allowing the use of `await` inside. | **Asynchronous function marker** | `async fn fetch_data() {}`, `pub async fn run() {}` |
| **`panics`** | Indicates that a function might exit unexpectedly by panicking (runtime error). | **Function may panic** | `fn divide(a: Int, b: Int) panics {}`, `fn unwrap() panics;` |
| **`mut`** | Modifier used to indicate that a variable or field can be mutated (changed). | **Mutable binding modifier** | `let mut counter = 0;`, `mut value: Int;` |
| **`final`** | Modifier indicating that a field cannot be reassigned after initialization. | **Immutable field modifier** | `final name: String;`, `final PI: Float = 3.14;` |
| **`as`** | Used for type casting or renaming imports. | **Type cast or rename** | `x as Float`, `* as io from "std::io"` |
| **`from`** | Used in import declarations to specify the source module path. | **Specify import source** | `from "lib::utils"`, `from "../file"` |
| **`try`** | Operator used to propagate errors from a potentially failing operation. | **Propagate error result** | `let data = try read_file();`, `try func()` |
| **`await`** | Pauses execution until the result of an asynchronous operation is available. | **Wait for async** | `await future;`, `await client.send()` |
| **`this`** | A reference to the current instance of a class or struct. | **Current object instance** | `this.field_name`, `fn method(this) {}` |
| **`ignore`** | A hint to the compiler to suppress warnings about an unused expression. | **Suppress unused warning** | `ignore call_function(x);`, `ignore result;` |
| **`let`** | Declares an immutable variable binding. | **Declare local variable** | `let x = 10;`, `let (a, b) = pair;` |
| **`const`** | Declares a constant variable binding that is typically known at compile time. | **Declare compile constant** | `const MAX_SIZE = 100;`, `const TIMEOUT: Int = 5;` |
| **`return`** | Exits the current function and optionally provides a value. | **Exit function, return** | `return 0;`, `return result;` |
| **`break`** | Exits the current loop. | **Exit current loop** | `break;`, `break value;` |
| **`continue`** | Skips the rest of the current loop iteration and proceeds to the next. | **Skip loop iteration** | `continue;` |
| **`if`** | Conditional statement for executing code based on a boolean expression. | **Conditional execution start** | `if x > 0 {}`, `if let Some(x) = opt {}` |
| **`else`** | Executes a block of code if the preceding `if` condition is false. | **Alternative condition branch** | `else {}`, `else if y < 0 {}` |
| **`match`** | Control flow for comparing a value against a sequence of patterns. | **Pattern matching control** | `match status {}`, `match value { 1 => x, _ => y }` |
| **`loop`** | Creates an unconditional, infinite loop. | **Unconditional infinite loop** | `loop {}`, `loop break 5;` |
| **`while`** | Creates a loop that continues as long as a condition is true. | **Conditional loop repetition** | `while i < 10 {}`, `while let Some(x) = iter {}` |
| **`for`** | Iterates over an expression (e.g., a collection or range). | **Iterate over elements** | `for item in list {}`, `for i in 0..10 {}` |
| **`in`** | Used with the `for` statement to indicate the iterable expression. | **Specify iterable source** | `for x in array`, `for key in map.keys()` |
| **`pipe`** | Defines a specialized block for chained operations using the pipeline operator. | **Define pipeline block** | `pipe data { ... }` |

---

## Built-in Type Names (Type Literals)

| Type Name | Description | Brief Description | Examples |
| :--- | :--- | :--- | :--- |
| **`Bool`** | A boolean type, either true or false. | **Boolean truth value** | `let flag: Bool = true;`, `fn check(): Bool` |
| **`Int`** | A signed integer, typically platform-dependent size. | **Signed integer type** | `let i: Int = 10;`, `fn get_count(): Int` |
| **`DoubleInt`** | A signed integer with double the bit size of `Int`. | **Double-size signed integer** | `let l: DoubleInt = 10000000000;` |
| **`Float`** | A single-precision floating-point number (e.g., 32-bit). | **Single-precision float** | `let f: Float = 3.14;`, `x as Float` |
| **`DoubleFloat`** | A double-precision floating-point number (e.g., 64-bit). | **Double-precision float** | `let d: DoubleFloat = 1.0e-10;` |
| **`Char`** | A single character, typically Unicode scalar value. | **Single character type** | `let c: Char = 'A';`, `match value { 'a' => 1, _ => 0 }` |
| **`Usize`** | An unsigned integer type used primarily for sizing collections or pointers, size is architecture-dependent. | **Unsigned size/index** | `let size: Usize = array.len();`, `for i in 0..N: Usize` |
| **`Any`** | A type that can hold a value of any type (often requires casting). | **Universal type container** | `fn returns_anything(): Any`, `let x: Any = 5;` |
| **`Never`** | The bottom type, representing the type of a function that never returns normally. | **Function never returns** | `fn exit() -> Never;`, `break value: Never` |
| **`Void`** | Represents the absence of a value, often used as the return type for functions that produce no value. | **Absence of value** | `fn log(m: String): Void`, `(): Void` |

---

## Operators and Delimiters

| Symbol | Description | Brief Description | Examples |
| :--- | :--- | :--- | :--- |
| **`{`** | Denotes the start of a code block or structure body (class, function, etc.). | **Begin block/body** | `fn main() {`, `if x { ... }` |
| **`}`** | Denotes the end of a code block or structure body. | **End block/body** | `... }`, `match x { ... }` |
| **`(`** | Denotes the start of a function call, parameter list, expression grouping, or tuple. | **Open group/call** | `func(1, 2)`, `(x + y)`, `(Int, String)` |
| **`)`** | Denotes the end of a function call, parameter list, expression grouping, or tuple. | **Close group/call** | `func(1, 2)`, `let t = (1, 2);` |
| **`[`** | Denotes the start of an array literal or the start of an indexing operation. | **Open array/index** | `[1, 2, 3]`, `array[i]` |
| **`]`** | Denotes the end of an array literal or the end of an indexing operation. | **Close array/index** | `[1, 2, 3]`, `array[i]` |
| **`,`** | Separates items in a list (arguments, parameters, fields, type parameters). | **List item separator** | `func(a, b)`, `x: Int, y: Float` |
| **`;`** | Terminates a statement or separates fields in a struct declaration. | **Statement terminator** | `let x = 5;`, `return value;` |
| **`:`** | Separates an identifier from its type or an initializer from a pattern/field in a struct. | **Type/value separator** | `x: Int`, `fn func(): Void`, `a: 10` |
| **`=`** | Used for variable initialization or type aliasing. (Note: The assignment operator is often a composite like `<ASSIGMENT_OPERATOR>`) | **Initializer/alias assignment** | `let x = 5;`, `type T = Int;` |
| **`?`** | Used in the optional return type of a function, or after `try`. | **Optional type/error** | `fn get()?: Int`, `try func()?` (Hypothetical) |
| **`@`** | Used for pattern binding in `match` or for annotations. | **Pattern binding/annotation** | `Some(x @ 5)`, `@Test annotation` |
| **`_`** | Wildcard pattern used to match anything or ignore a value. | **Wildcard/ignore pattern** | `match x { _, 5 => y }`, `let _ = func();` |
| **`|>`** | The pipeline operator, used to chain operations. | **Pipeline chain operator** | `data |> filter |> map` |
| **`=>`** | Separates the pattern from the expression in a `match` arm or pipeline. | **Pattern-expression separator** | `Some(x) => x`, `|>` `_` `=>` `x + 1` |
| **`::`** | The scope resolution or path separator operator. | **Scope resolution separator** | `std::io::File`, `MyClass::static_method` |
| **`&`** | Used for combining type constraints in generics or as a binary operator. | **Type constraint join** | `<T: Eq & Hash>`, `a & b` |
| **`<`** | Denotes the start of a generic parameter list or a binary less-than operator. | **Open generics/less-than** | `Class<T>`, `if x < y` |
| **`>`** | Denotes the end of a generic parameter list or a binary greater-than operator. | **Close generics/greater-than** | `Class<T>`, `if x > y` |
| **`.`** | The member access operator. | **Member access operator** | `object.field`, `array.len()` |
| **`..`** | The range operator (exclusive end). | **Exclusive range operator** | `0..10`, `for i in 1..N` |
| **`..=`** | The inclusive range operator. | **Inclusive range operator** | `0..=10`, `match x { 1..=5 => a }` |
| **`<ADD_LEVEL_BINARY_OPERATOR>`** | Placeholder for binary operators usually at the addition/subtraction level. | **Addition-level binary** | `+`, `-`, `|` |
| **`<MULTI_LEVEL_BINARY_OPERATOR>`** | Placeholder for binary operators usually at the multiplication/division level. | **Multiplication-level binary** | `*`, `/`, `%` |
| **`<UNARY_OPERATOR>`** | Placeholder for operators that take a single operand. | **Single operand operator** | `!`, `-`, `*` (dereference) |
| **`<ASSIGMENT_OPERATOR>`** | Placeholder for assignment operators. | **Variable assignment operator** | `=`, `+=`, `-=` |

---

## Literals and Identifiers

| Element | Description | Brief Description | Examples |
| :--- | :--- | :--- | :--- |
| **`<IDENTIFIER>`** | Names defined by the programmer for variables, functions, types, etc. | **Programmer defined name** | `my_variable`, `calculate_sum`, `User` |
| **`<STRING_LITERAL>`** | A sequence of characters enclosed in quotation marks. | **Sequence of characters** | `"hello world"`, `"path/to/file"` |
| **`<CHAR_LITERAL>`** | A single character enclosed in single quotation marks. | **Single character value** | `'a'`, `'\n'`, `'0'` |
| **`<NUM_LITERAL>`** | A numeric constant, including integers and floating-point numbers. | **Numeric constant value** | `42`, `3.14159`, `0xFF` |
| **`<BOOL_LITERAL>`** | The boolean constants: `true` and `false`. | **True/false value** | `true`, `false` |
| **`<DOCS_COMMENT>`** | Special comments used to generate documentation. | **Documentation generation comment** | `/// doc comment`, `//! doc module` |
