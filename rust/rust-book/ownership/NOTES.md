# Ownership
- keep track of what parts of code are using what data on the heap
- minimize the amount of duplicate data on the heap
- clean up unused data on the heap

## Memory Management Pattern
* garbage collection
* explicitly allocate and free the memory
* ownership: a set of rules that the compiler checks at compile time

## Stack and Heap
> both are parts of memory that is available to your code to use at runtime, but they are structured in different ways.

### Stack
* LIFO: last in, first out
* push onto/pop off the stack
* fast: no search, data with known, fixed size
* function parameters and local varibles

### Heap
> data with a size unknown at compile time or a size that might change can be stored on the heap instead.
> allocating on the heap: when you ask for some amount of space, the OS finds an empty spot in the heap, marks it as being in use, returns a pointer.
> slower accessing: you have to follow a pointer to get there 

## Ownership Rules
* each value has a variable called its owner
* only be one owner at a time
* the value will be dropped when the owner goes out of scope

## Variable Scope
> the range within a program for which a variable is valid

* when the variable comes into scope, it is valid
* it remains valid until it goes out of scope

## Memory and Allocation
> how to pair exactly one 'allocate' with exactly one 'free'
> RAII: Resource Acquisition Is Initialization, the dop function will be called in Rust

* ownership move: shallow copy
* clone: deep copy
* stack-only data copy

## Ownership and Functions
> passing a variable to a function will move or copy, just as assignments does

## Return Values and Scope
> returning values can also transfer ownership

## Copy Trait
* an older variable is still usable after assignment
* Copy trait is not allowed if it implements Drop trait

# References and Borrowing
> ampersand & is a reference, which allow you to refer to some value without taking ownership of it
> we call having references as function parameters borrowing

## Mutable References
* fn change(some_string: &mut String) {}
* change(&mut s)
* only one mutable reference to a variable in a particular scope

## Dangling References
> a pointer that references a location in memory that may have been given to someone else, by freeing some memory while preserving a pointer to that memory

## Rules of References
* at any given time, either one mutable reference or any number of immutable reference
* references must always be valid

# Slices
> let you reference a contiguous sequence of elements in a collection rather than the whole collection, which does not have ownership

## String Slices
> a reference to part of a String with .. range syntax

* &s[0..2], &s[..2], &s[3..len], &s[3..], &s[..]
* string literals are slices: &str
