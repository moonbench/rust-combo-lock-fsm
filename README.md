This is an example of an event-driven finite state machine in Rust, demonstrated by modeling a mechanical combination lock.

The states and transitions implemented by this look like:

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

There are three states: `UnlockedOpen`, `UnlockedClosed`, and `Locked`.

There are several events: `SetCode`, `Close`, `Open`, `Lock`, and `Unlock`.

