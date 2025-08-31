<!-- ANCHOR: McuCoreLocal -->
The main use-case behind 'core-local' (per-MCU core) data is that it provides an efficient means to access global state _without locking_, when that state is not required (or even desired) to be consistent or shared across cores.  Such use-cases are particularly useful for drivers and Interrupt Service Routines (ISRs) that need to store state between calls whilst the thread of execution has a natural core affinity.

For example, in a multi-core system, ISRs can often be enabled and disabled on a per-core basis.  For some devices, such as a DMA channel, it may not be useful to have every core interrupted, but some interrupt sources, such as an exception for a page fault or illegal instruction, the interrupt is specific to the code executing on the core.  If there is shared global state then locking or atomic operations are required to synchronise access, but often the global state does not need to be (or simply is never) shared across cores so does not require the extra synchronisation overhead.

There are only four ways to _safely_ expose core-local data:
1. The data is immutable for its entire lifetime and accesses therefore only require [`Copy`] semantics or shared references
   - Very efficient and simple but of limited use; most uses of core-local data will need mutation whilst avoiding locking
   - The stored type must not allow interior mutability, which cannot be enforced by generic bounds, although this could be enforced by restricting flexibility by allowing only a subset of 'safe' types
2. Only one ISR (or task in the main program with _single_ core affinity) accesses the data, regardless of whether that is via a shared or mutable reference
   - Inherently `unsafe` because there are no language-level constructs to convey and enforce these semantics; Rust does not model ISRs, for example
   - Does not work for tasks that do not have _single_ core affinity because a context-switch can occur at any time, such as between acquiring a mutable reference and using it
   - Obtaining a reference could allow it to be leaked and introduce _Undefined Behaviour_, for example if it was:
     - Stored in a `static` and mutated by the same ISR but running on another core
     - If a context switch occurs between obtaining a mutable reference and mutating it, but the task is resumed on another core
3. Use a _Critical Section_ to acquire both shared _and_ mutable references
   - ISRs are not allowed to block so a Semaphore, Mutex or other blocking construct is inappropriate; therefore interrupts need to be disabled (or equivalent)
   - Disabling interrupts can only be done via privileged execution; fine for ISRs but tasks may need a Syscall and a context-switch
   - The guard for the Critical Section needs to disable interrupts _for the current core only_
   - Only interrupts _that are higher priority_ than the current ISR (or main thread of execution) need to be disabled
   - This approach will not block but it does introduce latency and jitter when handling ISRs; this can break real-time guarantees
4. Only store data that can be manipulated with atomic semantics, such as a single machine word with a single Load-Link / Store-Conditional (LL / SC)
   - Mainly suited to smaller / simpler data-types, typically those that reside in a single machine word (`usize`), depending on machine architecture and intended operations
   - Potentially takes an indeterminate amount of time if the store fails (perhaps because a higher-priority ISR ran) and the operation has to be repeated
   - If a context-switch occurs during the operation then the SC will fail and the operation will need to be retried, but the retry will possibly be manipulating a different core, although the single atomic operation ought to be consistent if all values used in the computation are all in the same block.  Tasks without _single_ core affinity should generally not be using core-local state as it is of limited utility, although some algorithms (such as pseudo-random number generators) could put it to good use.
   - LL / SC only covers a single read-write - multiple field updates would not be possible, even if Rust could be coerced into using LL / SC for every field access, because the first LL / SC could execute successfully on core N immediately before a context-switch that causes the second LL / SC to execute (and succeed) on another core

Option 1 is very easy to implement as long as the types do not leverage interior mutability - this cannot be enforced by Rust, however.

Option 2 is of limited utility, since core-local storage used by a single task (assuming the task never completes) is the same as using a value from the tasks's stack, which is never shared and doesn't require locking or other special treatment.  For an ISR it can be useful, since ISR code is shared and can be executing concurrently across multiple cores.  Potential use-cases for tasks are algorithms such as a pseudo-random number generator or entropy source, so there could be a need.

Options 3 and 4 can be implemented with a guard and a closure to take advantage of Rust's borrow-checker and scoping rules to make sure references do not leak beyond the scope containing the access.  In the case of the Critical Section solution, however, [`Drop`] _must_ be run, otherwise the stability of the whole system can be compromised when ISRs are disabled and never re-enabled.  If [`Drop`] is not run for atomic stores then the worst that will happen is the value will not be stored, which may or may not have wider-ranging consequences for system stability.

Further complications arise when it is realised that, for a task without _single_ core affinity, between retrieivng the ID of the current core and calling the supplied closure, it is possible that a context-switch occurred.  If the reference supplied to the closure uses interior mutability then one core is modifying another core's data and causing _Undefined Behaviour_.  For a task _with single_ core affinity, the context-switch could still cause problems if the other task that was switched in to run on the core also manipulated the same data, leaving a potentially invalid state in memory or falling afoul of compiler assumptions around optimisation and re-ordering, thus also leading to _Undefined Behaviour_.

*Other than immutable option 1 above, there is _no safe way_ for tasks to manipulate core-local data without disabling higher-priority interrupts to prevent context-switching or prevent any interrupts manipulating the same core-local state, since interrupts are not able to leverage locking.*

*Core-Local Storage should therefore only be used from within an ISR context.*  If ISRs and tasks need to communicate then it should be via a non-core-local mechanism such as message-passing.

## Undefined Behaviour
Based on the points and challenges outlined in the above section, it is considered _Undefined Behaviour_ if any of the following occurs:
- A core-local value is accessed (load or store) by code executing on another core unless:
  - the data is immutable _in its entirety (ie. graph)_ for the entire lifetime of the system, or
  - the operation is an _atomic load_ and the entire core-local slot is _a single atomic datatype_; stores are prohibited and locking is not a mitigation
- A core-local value is accessed by any code outside of an ISR context (ie. a task), regardless of core affinity; bootstrapping and kernel initialisation, prior to enabling interrupts, is considered an ISR context (triggered via the reset vector)
- The stored datatype allows interior mutability
- For mutable datatypes, a pointer or reference to a core-local value is used outside of the closures that manipulate it

The above list is non-exhaustive but represents an obvious starting-point of things to avoid.

## Implementation
The implementation has been modelled on [`std::thread::LocalKey`] and associated functionality because this is an idiomatic Rust approach to managing thread synchronisation.

That the process of determining the ID of the current core and then looking up and loading the appropriate core-local slot is not an atomic operation.  Outside of ISRs this would require locking, which is potentially unsound if a context-switch evicts a task from a given core and it is not able to be scheduled back to the same core.  ISRs cannot be allowed to block, so task-ISR synchronisation becomes non-trivial, so instead we prohibit core-local storage in anything other than an ISR context.  ISRs can be relied upon to execute to completion on the same core even if they are able to be pre-empted, meaning that this non-atomic process of 'get core ID / load slot from memory' does not require any special guards providing the access to the slot data itself is atomic.
<!-- ANCHOR_END: McuCoreLocal -->

<!-- ANCHOR: McuCoreLocal.new_all -->
Create a new `McuCoreLocal` instance with all cores having the same initial value.

The lifetime of the `McuCoreLocal` cannot outlive the given `mcu` (usually a `static` or a driver-level construct).  It is up to the caller to verify that the datatype `T` does not cause _Undefined Behaviour_, as discussed above.
<!-- ANCHOR_END: McuCoreLocal.new_all -->

<!-- ANCHOR: McuCoreLocal.new -->
Create a new `McuCoreLocal` instance with each core getting its own initial value.

The lifetime of the `McuCoreLocal` cannot outlive the given `mcu` (usually a `static` or a driver-level construct).  It is up to the caller to verify that the datatype `T` does not cause _Undefined Behaviour_, as discussed above.
<!-- ANCHOR_END: McuCoreLocal.new -->

<!-- ANCHOR: McuCoreLocal.with -->
*ISR CONTEXT ONLY* Access, and potentially manipulate, the stored value for the current core.

*IT IS UNDEFINED BEHAVIOUR TO CALL THIS FROM OUTSIDE OF AN ISR CONTEXT*

The given function is executed with the current core-local value.  The value returned from the function will be passed back to the caller of `with()`.  It is up to the caller how to use and manipulate the value inside the function / closure whilst avoiding _Undefined Behaviour_.

Note that [`despair!`][smeg_kernel::despair] can happen if the ID for `mcu.mcu_core_id()` is out of range for the number of cores.  This should never happen since it is a violation of the `HasMcuCoreId` contract, but the check is required to maintain safety guarantees.
<!-- ANCHOR_END: McuCoreLocal.with -->
