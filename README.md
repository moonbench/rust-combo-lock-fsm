This is an interactive example of an event-driven finite state machine in Rust, demonstrated by modeling a mechanical combination lock.

## The state machine

The states and transitions implemented by this look like:

![State machine diagram](https://moonbench.xyz/assets/images/projects/rust_state_machine/lock_states.png)

There is a shared `combination` value in the machine which is an array of `i8`s.

There are three states: `UnlockedOpen`, `UnlockedClosed`, and `Locked`.

There are five events: `SetCode`, `Close`, `Open`, `Lock`, and `Unlock`. And the states each respond to a subset of the events.

It can be represented textually like:
```
   ______________                ________________                 ________
  |              | -- CLOSE --> |                |               |        |
  |   Unlocked   |              |    Unlocked    | --- LOCK ---> | Locked |<-------.
  |     Open     |              |     Closed     |               |        |        |
  |______________| <-- OPEN --- |________________|               |________|        |
       |    ^                           ^                           |              |
       |    |                           |                    UNLOCK |         (NO) |
       \____/                           |                    _______V__________    |
                                        |                   /                  \   |
      SET CODE                          \___________________| Is Correct Code? |___/
                                                    (YES)   \__________________/
```

## Interactive

This demo has an interactive command-line component to let the user interact with the state machine.

There are no depedndencies besides Rust, so simply download the code and run it with `cargo run`.

You can then interact with the machine. Type `help` for a list of all of the commands.

```
Finite State Machine Lock Demo

         ________
        |  ____  |
        | |    | |
        |_|    | |
         ______|_| OPEN
        |   __   |
        |  [  ]  |
        |   []   | UNLOCKED
        |   []   |
         \______/

Enter command: set 11 12 13
[DONE] Code changed
         ________
        |  ____  |
        | |    | |
        |_|    | |
         ______|_| OPEN
        |   __   |
        |  [  ]  |
        |   []   | UNLOCKED
        |   []   |
         \______/

Enter command: close
[DONE] Lock closed
         ________
        |  ____  |
        |_|____|_| CLOSED
        |   __   |
        |  [  ]  |
        |   []   | UNLOCKED
        |   []   |
         \______/

Enter command: lock
[DONE] Locked
         ________
        |  ____  |
        |_|____|_| CLOSED
        |   __   |
        |  [**]  |
        |   []   | LOCKED
        |   []   |
         \______/

Enter command: open
[ERROR] Not allowed for a locked lock. Unlock it first.
         ________
        |  ____  |
        |_|____|_| CLOSED
        |   __   |
        |  [**]  |
        |   []   | LOCKED
        |   []   |
         \______/

Enter command: unlock 0 0 0
[ERROR] Invalid combination, still locked
         ________
        |  ____  |
        |_|____|_| CLOSED
        |   __   |
        |  [**]  |
        |   []   | LOCKED
        |   []   |
         \______/

Enter command: unlock 11 12 13
[DONE] Valid combination, unlocked!
         ________
        |  ____  |
        |_|____|_| CLOSED
        |   __   |
        |  [  ]  |
        |   []   | UNLOCKED
        |   []   |
         \______/

Enter command: open
[DONE] Lock opened
         ________
        |  ____  |
        | |    | |
        |_|    | |
         ______|_| OPEN
        |   __   |
        |  [  ]  |
        |   []   | UNLOCKED
        |   []   |
         \______/

```

A post with more details: https://moonbench.xyz/projects/rust-event-driven-finite-state-machine/
