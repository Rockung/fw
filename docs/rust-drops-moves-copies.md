## Drops, Moves, and Copies

> deterministic and implicit destruction of objects
> ownership of objects
> assignment semantics: share, copy, and move
> implicit share semantics is bad for software correctness
> move semantics may have better performance than copy semantics
> decide when some types need copy semantics
> decide when some types need to be non-cloneable

### Deterministic Destruction
> allocated in the stack
		temporary expressions
		variables(including arrays)
		function and closure arguments
		box-es, dynamic strings, and collections headers
> allocated in the heap
		referenced objects
		data of dynamic strings and collections(including vectors)
> deallocated
	temporary expressions: when the statement containing them ends
	variables: when the scope containing their declaration ends
	function/closure arguments: when their block ends
	boxed objects: when the scope containing their declaration ends
	chars in dyna strings: when they are removed from the string, or when the string is deallocated
	items in collections: when they are removed from the collection, or when the collection is deallocated

In rust, heap deallocation is both deterministic and implicit
	 
*non-deterministic*
> it happens in unknown instants of execution
*implicit*
> it does not require specific deallocation statements

### Ownership
A owns an object B, means that A is responsible for deallocating B
> only A can deallocate B
> when A becomes unreachable, A must deallocate B
> every object must have exactly one owner, no more, no lesss

let mut a = 3;
a = 4;
> a is the owner of an object whose initial value is 3
> its ownership hasn't changed, but the value of the object is changed to 4

let b = vec![11, 22, 33, 44, 55]
> b owns the header of a vector
> the pointer in the header owns the data buffer
> the data buffer contains the five items

let a = 3;
{
	let a_ref = &a;
} 
print!("{}",a);
> a_ref owns a reference which does not own anything
> the referenced object shouldn't be immediately deallocated

### Destructors
> objects are destroyed in exactly the opposite order than their construction order
> temporary objects are destroyed at the end of their statements
> the syntax to implement the trait Drop for the newly declared type
impl Drop for ... {
	fn drop(&mut self) {}
}

### Assignment Semantics
> three types: share, copy, move
> the default semantics is a move. to make a copy, invoke clone()
> copy semantics: numbers, booleans, static strings, arrays, tuples, and references
> move semantics: dynamic strings, boxes, collections, enums, structs, tuple-structs

### Cloning Objects
> copy semantics: cloneable and copyable, no heap objects, no external resources
> move semantics: cloneable but non-copyable, heap objects, no external resouces
> move semantics: non-cloneable and non-copyable, external resources

