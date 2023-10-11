# Rust

## Cargo

Cargo is Rust's package manager.
cargo new name will create a new Rust project with the name "name".
cargo build will create an executable, but will not run the code.
cargo run will create both create an executable and run the code.

## Memory Management

### The Stack

The Stack is a special portion of memory that stores the variables created by functions.
The Stack is automatically managed.
The Stack has a size limit.
The memory required for variables is created as the function executes. 
When the function returns, the variables within fall out of scope, and the stack memory for those variables is freed.
Every function call will create a new entry to the Stack, called a stack frame, with its associated variables.
Compiling converts code from source code to machine code in a series of steps.
If the size of a variable is known at compile time, it can be stored on the Stack.

### The Heap

The Heap is a section of memory used to store data that:
1. Needs to persist beyond a particular scope.
2. The size of the data is unknown at compile time, or the size of the data may change during the course of a program.
3. The data is boxed using a smart pointer.
4. You want to share ownership of the data across multiple parts of the program.
Data stored on the heap does not have a size limit, except for the total memory available on the computer.
Variables on the heap are available to any function at any time.

## Key Language Features

Rust has strongly typed data. 
This enables the following features:
1. Type enforcement: Rust prevents operations that result in type mismatches or violations.
2. Type safety: Rust's compiler catches errors at compile time.
3. Excplicit type conversion: Operations that require type conversions must be done explicitly.
4. Type inference: Rust's compiler will deduce what type of variable is being declared.

## Scalar Data Types

Rust has four primary scalar data types:

### Integers

Integers in Rust can be typed as i8 - i128, or for signed integers, u8-u128. 

### Floating Point

Floating point numbers in Rust can be typed as f32 or f64. 
By default, Rust types floating point numbers as f64. 
All floating point numbers are signed.

### Boolean 

Boolean values are typed as bool.

### Char

Char values in rust are 4 bytes in size and represent a Unicode Scalar value. 
Note that chars are represented by single quotes, while strings are represented by double quotes.

## Compound Data Types

Rust has two primitive compound types:

### Tuples

Tuples in Rust are fixed size collections of primitive values. 
A tuple can contain different types of primitives. 
Tuple literals can be defined with ( ), for example, let tuple = ("hello", 5, 'h');
The type of a tuple is simply the type of each of its consitutent values.
The values of a tuple can be access with dot notation, followed by an index, for example: tuple.0 = "hello"

### Arrays

Arrays in rust are a fixed size collection of primitive values. 
Unlike tuples, all of the values in an array must be of the same type, since the same amount of space is allocated for each scalar value in an array.
Array literals can defined with brackets, for example: let array = [1, 2, 3, 4];
The values of an array can be accessed with bracket notation: 

### Strings

#### UTF-8

With ASCII character encoding, each character is represented by a fixed size byte of data.
This limits the total number of characters ASCII can represent to 256. 
As a result, Rust developers chose to represent characters with UTF-8 encoding. 
UTF-8 encoding is variable length encoding, using between 1 and 4 bytes to represent all used characters in all languages.

### String

The String is heap allocated, growable, and UTF-8 encoded. 
Strings are an owned data type.
When the variable that owns the String goes out of scope, the underlying data is deallocated.
A string variable consists of a pointer to the start of the string on the heap, information on the length of the string, and the capacity of the string.
Strings in Rust do not use a termination character.
String types are useful when you want to create or modify string data at runtime.

### String Slices

String slices are borrowed reference to a part of a string.
Think of a string slice as a view into existing string data.
Pointers to string slices store information about the length of the string, but they do not store information about capacity, since they are not growable.
String slices are useful when you want to read or analyze existing string data without making changes to it.

### &'static str

Static string slices are very common. 
The static lifetime modifier causes a string literal, which is usually a simple string slice, to exist for the lifetime of the program.
In order to ensure that this data exists for the lifetime of the program, this &' static str gets baked into the binary for the program in a special "static data segment."

## Variables

Variables declared with the let keyword can be mutable with the mut keyword.
They do not need a type annotation.
Variables can be initialized without a value using the let keyword, however, they need a type specified.
Variables declared with the const keyword must have a type and value assigned.
Variables "own" their values.
Since Rust has strong typing, once the type of a variable has been declared you cannot change a variable's type.
Rust allows shadowing, or redeclaring the same variable -- potentially with a different type -- within a nested scope.

## Functions

Use the fn keyword to define a new function.
Function signatures must have a type annotation after each parameter.
The return value of functions in Rust must be known at compile time.
If a function has a return, it must include a return type in the function signature.
In Rust, expressions return a value based on the operands, whereas statements are lines of code that perform an operation but don't return a value.
The final return line of a function can be an expression without a ";".


## If / Else
Use the following syntax to declare if / else blocks;

if condition { value } else { othervalue };

or

if condition {
    return value;
} else {
    return othervalue;
};

In Rust, 

The results of if / else blocks can be assigned to values. 
When the results are assigned to values, all branches of the if else block must evaluate to the same type.