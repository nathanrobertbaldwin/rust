# Rust

## Memory Management

### The Stack

The Stack is a special portion of memory that stores the variables created by functions. 
Every function call will create a new entry to the Stack, called a stack frame, with its associated variables.
Compiling converts code from source code to machine code in a series of steps.
The size of variables on the stack must be known at runtime, so the stack can only contain scalar data types.

### Scalar Data Types

Rust has four primary scalar data types:

#### Integers

Integers in Rust can be typed as i8 - i128, or for signed integers, u8-u128. 

#### Floating Point

Floating point numbers in Rust can be typed as f32 or f64. 
By default, Rust types floating point numbers as f64. 
All floating point numbers are signed.

#### Boolean 

Boolean values are typed as bool.

#### Char

Char values in rust are 4 bytes in size and represent a Unicode Scalar value. 
Note that chars are represented by single quotes, while strings are represented by double quotes.

### Compound Data Types

Rust has two primitive compound types:

#### Tuples

Tuples in Rust are fixed size collections of primitive values. 
A tuple can contain different types of primitives. 
Tuple literals can be defined with ( ), for example, let tuple = ("hello", 5, 'h');
The type of a tuple is simply the type of each of its consitutent values.
The values of a tuple can be access with dot notation, followed by an index, for example: tuple.0 = "hello"

#### Arrays

Arrays in rust are a fixed size collection of primitive values. 
Unlike tuples, all of the values in an array must be of the same type, since the same amount of space is allocated for each scalar value in an array.
Array literals can defined with brackets, for example: let array = [1, 2, 3, 4];
The values of an array can be accessed with bracket notation: 

