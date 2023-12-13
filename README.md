This is an example of an event-driven finite state machine in Rust, demonstrated by modeling a mechanical combination lock.

The states and transitions implemented by this look like:

```
   ______________                ________________                 ________
  |              | -- CLOSE --> |                | --- LOCK ---> |        |
  | UnlockedOpen |              | UnlockedClosed |               | Locked |
  |______________| <-- OPEN --- |________________| <-- UNLOCK -- |________|
```

