<!-- ANCHOR: ReadWriteCell -->
*DO NOT CREATE REFERENCES*.  A memory cell that can be read from and written to by the application.

An opaque `struct` representing a memory cell with (potentially) architecture-specific semantics.  It is _Undefined Behaviour_ to allow *any* reference to an
instance of this type, unless that instance can be _guaranteed_ to exist only in 'normal' memory with Rust-compatible semantics.  For example, it is safe and
well-defined behaviour to instantiate this `struct` on the stack of a function for the purposes of unit testing, but instant _Undefined Behaviour_ if any part
of the codebase takes a reference to an instance defined in a linker script.

The purpose of this `struct` is _not_ to allow dereferencing or access to the contents of memory, but to provide a mechanism for reserving space and defining
memory layouts.  For example, a device may expose registers that are assigned addresses in a linker script; those symbols are then available as (typed) labels
for Rust code to manipulate.  The labels can be passed to architecture-specific Memory-Mapped I/O (MMIO) routines, probably written in assembly language, so
long as no references to the same memory addresses exist.

This `struct` has the same size and alignment as the underlying type `T`, as well as a set of architecture-specific [`MemoryAttributes`] that can be used when
determining what operations a corresponding [`CellAccessor`] can provide.

Use a [`CellAccessor`] to safely take references to, encapsulate and otherwise manipulate the contents backed by this opaque representation of a Cell.
<!-- ANCHOR_END: ReadWriteCell -->

<!-- ANCHOR: ReadonlyCell -->
*DO NOT CREATE REFERENCES*.  A memory cell that can only be read from, not written to, by the application.

An opaque `struct` representing a memory cell with (potentially) architecture-specific semantics.  It is _Undefined Behaviour_ to allow *any* reference to an
instance of this type, unless that instance can be _guaranteed_ to exist only in 'normal' memory with Rust-compatible semantics.  For example, it is safe and
well-defined behaviour to instantiate this `struct` on the stack of a function for the purposes of unit testing, but instant _Undefined Behaviour_ if any part
of the codebase takes a reference to an instance defined in a linker script.

The purpose of this `struct` is _not_ to allow dereferencing or access to the contents of memory, but to provide a mechanism for reserving space and defining
memory layouts.  For example, a device may expose registers that are assigned addresses in a linker script; those symbols are then available as (typed) labels
for Rust code to manipulate.  The labels can be passed to architecture-specific Memory-Mapped I/O (MMIO) routines, probably written in assembly language, so
long as no references to the same memory addresses exist.

This `struct` has the same size and alignment as the underlying type `T`, as well as a set of architecture-specific [`MemoryAttributes`] that can be used when
determining what operations a corresponding [`CellAccessor`] can provide.

Use a [`CellAccessor`] to safely take references to, encapsulate and otherwise manipulate the contents backed by this opaque representation of a Cell.
<!-- ANCHOR_END: ReadonlyCell -->

<!-- ANCHOR: WriteonlyCell -->
*DO NOT CREATE REFERENCES*.  A memory cell that can only be written to, not read from, by the application.

An opaque `struct` representing a memory cell with (potentially) architecture-specific semantics.  It is _Undefined Behaviour_ to allow *any* reference to an
instance of this type, unless that instance can be _guaranteed_ to exist only in 'normal' memory with Rust-compatible semantics.  For example, it is safe and
well-defined behaviour to instantiate this `struct` on the stack of a function for the purposes of unit testing, but instant _Undefined Behaviour_ if any part
of the codebase takes a reference to an instance defined in a linker script.

The purpose of this `struct` is _not_ to allow dereferencing or access to the contents of memory, but to provide a mechanism for reserving space and defining
memory layouts.  For example, a device may expose registers that are assigned addresses in a linker script; those symbols are then available as (typed) labels
for Rust code to manipulate.  The labels can be passed to architecture-specific Memory-Mapped I/O (MMIO) routines, probably written in assembly language, so
long as no references to the same memory addresses exist.

This `struct` has the same size and alignment as the underlying type `T`, as well as a set of architecture-specific [`MemoryAttributes`] that can be used when
determining what operations a corresponding [`CellAccessor`] can provide.

Use a [`CellAccessor`] to safely take references to, encapsulate and otherwise manipulate the contents backed by this opaque representation of a Cell.
<!-- ANCHOR_END: WriteonlyCell -->

<!-- ANCHOR: CellAccessor -->
The preferred mechanism for encapsulating a [`Cell`] and providing appropriate access to its contents.

The contents of a [`Cell`] cannot be accessed without knowing architecture-specific details about the memory, nor can a [`Cell`] have a reference taken without
invoking _Undefined Behaviour_.  The purpose of this `struct` is encapsulation and specialisation - it allows references to be taken and passed around for any
type of [`Cell`] whilst also allowing architecture-specific traits to be implemented that can manipulate the type of memory that is backing the [`Cell`].
<!-- ANCHOR_END: CellAccessor -->

<!-- ANCHOR: CellAccessor.new -->
Store a pointer to a [`Cell`].

Since references cannot be used to refer to a [`Cell`] without introducing _Undefined Behaviour_, a `CellAccessor` encapsulates a pointer.
<!-- ANCHOR_END: CellAccessor.new -->

<!-- ANCHOR: CellAccessor.get -->
Get a pointer to the contents of a [`Cell`].

Use of this method is discouraged for anything other than the architecture-specific traits that encapsulate memory accesses.
<!-- ANCHOR_END: CellAccessor.get -->
