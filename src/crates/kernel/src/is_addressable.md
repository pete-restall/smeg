<!-- ANCHOR: IsAddressable -->
Trait to determine whether an immutable pointer is addressable for reading.

Provides an abstraction for determining whether a pointer is addressable in a given immutable context, for example if a pointer is within a Task's address space.

Note that being _addressable_ does not mean the pointer is valid and can be dereferenced without _Undefined Behaviour_.  Being _addressable_ just means that the pointee is correctly aligned and within an address space.  Unless the `struct` implementing this trait stores a list of valid pointers, there is in general no way to know whether, for example, a pointee:
* that is otherwise correctly aligned and sized straddles more than one allocation block
* contains uninitialised memory
* is of a type different to the pointer
* is offset from the start of the `struct`
* is dangling or has an incompatible lifetime
* has another (mutable) reference to it
* or any of the other plethora conditions that would cause _Undefined Behaviour_.
<!-- ANCHOR_END: IsAddressable -->

<!-- ANCHOR: IsAddressable.is_addressable -->
Does the given pointer address an immutable value ?
<!-- ANCHOR_END: IsAddressable.is_addressable -->

<!-- ANCHOR: IsAddressableMut -->
Trait to determine whether a mutable pointer is addressable for writing.

Provides an abstraction for determining whether a pointer is addressable in a given mutable context, for example if a pointer is within a Task's address space.

Note that being _addressable_ does not mean the pointer is valid and can be dereferenced without _Undefined Behaviour_.  Being _addressable_ just means that the pointee is correctly aligned and within an address space.  Unless the `struct` implementing this trait stores a list of valid pointers, there is in general no way to know whether, for example, a pointee:
* that is otherwise correctly aligned and sized straddles more than one allocation block
* contains uninitialised memory
* is of a type different to the pointer
* is offset from the start of the `struct`
* is dangling or has an incompatible lifetime
* has another mutable reference to it
* or any of the other plethora conditions that would cause _Undefined Behaviour_.
<!-- ANCHOR_END: IsAddressableMut -->

<!-- ANCHOR: IsAddressableMut.is_addressable_mut -->
Does the given pointer address a mutable value ?
<!-- ANCHOR_END: IsAddressableMut.is_addressable_mut -->
