<!-- ANCHOR: module -->
// strategy for cortex m4:
// set sv_call and pend_sv interrupts to identical lowest possible priority (!0); do this in the syscall driver
// three syscalls:
//   - yield; simply sets pend_sv
//   - exit(ExitArgs); exits the task - the 'ExitArgs' can be interpreted by the context-switching code via a (Driver-supplied) 'on_exit' hook or something similar, maybe to load an overlay or co-operative task or queue a message or something
//   - run(RunArgs); runs a new task - need to think about this one; should we allow a flag to exit the current task first (to allow stack re-use, for example - useful for the scheduler / idle task).  What happens if the task is already running ?  How about if it's already running at the same priority - do we yield, or resume to allow a timeslice to continue ?  Each task must only have one instance, however - cannot start multiple identical tasks.
//
// one isr - pend_sv simply sets current task to next task (or scheduler task if none) and restores context for that task
// scheduler task runs unprivileged (one per core) and populates / shuffles task queues so 'next task' is populated for next 'yield', which scheduler does via 'syscall yield'
// the scheduler ISR will be common for every architecture; the scheduler task can be feature-toggled for different algorithms, etc.
// even the lowest priority ISR pre-empts thread-mode execution (unless privileged thread-mode raises its priority, but then lower priority ISRs will not pre-empt it, so effectively thread-mode is always the lowest priority)
//

/*
overview

pre-emptive tick isr:
  - feature-toggled
  - does not signal a context-switch (pend_sv on arm) if the idle task or scheduler is already running
  - does not signal a context-switch if the runnable mask is 0 or only has a single bit set (encompasses the above condition)
  - can be used to implement a soft watchdog for health-checking the scheduler (see below)

task-switching isr:
  - other isrs can request a re-scheduling by raising the desired execution priority; to play nicely they must never lower it otherwise scheduling may not occur
  - the desired execution priority is reset to 0 after each successful scheduling
  - if desired execution priority >= current priority then the priority queue / round-robin bitmask need to be zeroed, so the task scheduler will never share its priority
  - starts (or forces restart of) the scheduler task if current task bitmask is 0 (ie. all 32 tasks in current priority queue are blocked)
  - scheduler task takes the priority of the 'desired execution priority' but has no task queue entry, so if an ISR raises this level when a scheduler is running then the scheduler task must be restarted
  - scheduler task can block (eg. do syscalls); to prevent deadlocks then a watchdog should be implemented to restart the scheduler (feature-toggled via config value - needs to be implemented in timer interrupt, however, so only available to tick-based builds)
  - if the scheduler needs to be killed 'N>=1' times in a row because of deadlocks then despair (feature-toggled via config value; N=0 means no despair test)
  - when the scheduler task has terminated, the current priority and the available task bitmask will be set in the task's address space for the isr to copy without sync issues
  - if no task scheduling is necessary then barrel-shift the priority queue to the next available task and switch to it, ie. efficient round-robin via 'clz'

scheduler task:
  - started by task-switching isr (pend_sv)
  - runs to completion; no special syscalls to manipulate the task queues (that could be leveraged or abused by other tasks)
  - just a regular task that can be pre-empted (by ISRs; no pre-emption by other tasks, since there should be no others in the runnable state)
  - not part of the priority queues, much like the idle task - these two are special tasks and are started on demand
  - on yield or exit, (pend_sv) isr can inspect state of queues - we'll need some sort of ping-pong buffer to flag / change ownership between isr and task
  - if there is no unblocked task at the desired priority level then search the next lowest priority queue and so on
  - if there are no tasks to execute then queue an idle task and exit and the whole (pend_sv) isr / scheduling process should run again, in an infinite loop until something becomes schedulable
  - if there is more than one core then some algorithm will have to be created to shuffle tasks between cores, which gets complicated (tasks of the same priority running and moving between cores, etc.); do not worry about this for the first iteration, each core will manage its own subset of tasks

Note that it is very inefficient if a runnable task at the current priority becomes unrunnable, since we cannot just clear the bit in the runnable mask
because that could happen, for example, right before the isr returns, in which case we're returning to an unrunnable task.  Although, if we keep this
atomic, perhaps we can allow a higher-priority isr to set or clear the runnable flags _if_ the pend_sv flag is set first, guaranteeing the isr runs again - but
this is bad because they could set invalid bits.  Maybe a better approach is to allow isrs a means to communicate what sort of scheduling change they have
caused, eg. task state change, but again, they could interfere with other bits.  Maybe best, in the task scheduler isr, to use something like
'if desired_priority == current_priority { iterate_all_tasks_and_rebuild_mask() } else { invoke_full_task_scheduling() }

```ignore
// some parts of the state need to be shared across ISRs, thus need to be atomic (eg. desired execution priority)
// some parts of the state are only used inside a single ISR, this no locking or pre-emption protection is required (eg. the runnable task mask)
struct TaskSchedulerIsrState {
    desired_task_priority: McuCoreLocal::new_all(&mcu, AtomicUsize::default());
    runnable_task_mask: McuCoreLocal::new_all(&mcu, xxxCell::<RoundRobinUsizeTaskMask>::default());
};


let let Some(next_task_index) = runnable_task_mask.with(|mask| mask.get_mut().next()) {
    //
} else {
    // no runnable tasks
}


// smells like an iterator !  (but isn't)
// maybe it's another trait though, to allow a different algorithm that RR to be used ?

struct RoundRobinUsizeMask {
    mask: usize,
    index: u8
}

impl RoundRobinUsizeMask {
    pub fn reset(&mut self, mask: usize) { // this isn't right - we should continue from the same index to keep time slicing more fair
        self.mask = mask;
        self.index = 0;
    }

    pub fn next(&mut self) -> Option<usize> {
        let number_of_zeroes = self.mask.count_leading_zeroes();
        self.mask = self.mask.rotate_left(number_of_zeroes + 1);
        self.index = (self.index + number_of_zeroes) & (usize::BITS - 1);

        if number_of_zeroes < usize::BITS {
            Some(self.current_task_index)
        } else {
            None
        }
    }
}
```
*/

/*
tasks have priorities - priorities are static but configurable

per-core runnable queues
  one queue per priority (up to usize::BITS priorities, up to usize::BITS tasks per queue)
  feature-toggle

global runnable queues
  one queue per priority (up to usize::BITS priorities, up to usize::BITS tasks per queue)
  feature-toggle

at least one set of queues must be available


// can any isr other than pend_sv block a task, or simply unblock a task ?  if only pend_sv can block a task then 'clz' approach works, otherwise it is just a HINT (since a higher-priority ISR could clear the flag after clz has run, thus leaving an entire queue devoid of runnable tasks)


// Only allow up to `usize::BITS` task priorities:
//   task_priority_ready_flags: usize
//
// Then we can:
//   1. logically OR a task_control_block's (const) priority when it is marked as runnable - pend_sv flag must be set immediately after this
//   2. use 'clz' when task-switching to find which (highest) priority list has a runnable task

// only allow up to 256 tasks per priority
task_index_per_priority: u8[P]

//
tasks_per_priority[0]: task_control_block[M]
...
tasks_per_priority[P]: task_control_block[N]



when task becomes runnable:

*/
<!-- ANCHOR_END: module -->
