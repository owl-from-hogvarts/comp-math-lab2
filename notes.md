
- interrupt handler receives args through shared memory (!!!)
- only single instace of interrupt handler can run at a time

consequent calls to write should not override value of shared location

interrupt handler executes multiple times and drops value last time it is called

value is owned by cell
interrupt handler borrows value for each execution
main thread may set new value, only when cell is empty

only interrupts can borrow value. they may borrow mutably

configuration can be updated only in `Idle` state

