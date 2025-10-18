<!-- ANCHOR: BankAccessor -->
The preferred mechanism for encapsulating a [`Bank`] and providing appropriate access to its contents.

The contents of a [`Bank`] cannot be accessed without knowing architecture-specific details about the memory, nor can a [`Bank`] have a reference taken without
invoking _Undefined Behaviour_.  The purpose of this `struct` is encapsulation and specialisation - it allows references to be taken and passed around for any
type of [`Bank`] whilst also allowing architecture-specific traits to be implemented that can manipulate the type of memory that is backing the [`Bank`].
<!-- ANCHOR_END: BankAccessor -->

<!-- ANCHOR: BankAccessor.new -->
Store a pointer to a [`Bank`].

Since references cannot be used to refer to a [`Bank`] without introducing _Undefined Behaviour_, a `BankAccessor` encapsulates a pointer.
<!-- ANCHOR_END: BankAccessor.new -->
