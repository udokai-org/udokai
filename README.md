# Udokai - The launcher for unix-like systems

**THIS IS VERY MUCH A WORK IN PROGRESS.**

[Architecture](https://excalidraw.com/#json=GmWGK8vX4JbHzk3Mue1ka,uMhpuff-yz6ABxDl2o4R6w)

## Sequence Diagram

The client and the server(s) process(es) will always be running in the background, to ensure it's snapyness to respond to the user's input. The client will forward the user's input to the servers, and if matches one or more of their trigger will respond with a list of items. The client is responsible for aggregate the responses

```mermaid
sequenceDiagram
    autonumber
    participant GUI/TUI
    participant Client
    participant Server1
    participant Server2
    participant OperationSystem

    GUI/TUI->>Client: Keypress Event 
    Client->>Server1: Forward Keypress Event
    Client->>Server2: Forward Keypress Event
    Server1-->>Client: Processed Response (e.g., "Action1")
    Server2-->>Client: Processed Response (e.g., "Action2")
    Client-->>GUI/TUI: Aggregated Responses ("Action1", "Action2")
    GUI/TUI->>Client: Select an action (Action1)
    Client->>Server1: Send selected action (Action1)
    Server1->>OperationSystem: Execute Action1
```

## TODOs / FEATURES

- [x] Implement a basic client/server communication using UnixSocket
- [x] Implement a basic TUI using ratatui for testing on the terminal
- [x] Communication between TUI and Client via Stdin/Stdout
- [ ] Handle server responses aggregation and display
- [ ] Handle client command to server (selecting an item)
- [ ] Implement a basic GUI using iced to use as the main interface
- [ ] Implement protocol of communication between client and server
    - [ ] Implement protocol for server to register triggers
    - [ ] Implement protocol for client to send user input
    - [ ] Implement protocol for server to respond with a list of items
    - [ ] Implement protocol for client to send command to server to execute
- [ ] Implement protocol for client to discover servers running dynamically (config? auto-discovery?)

### Default Servers

- [ ] Implement a server that return the list of Applications installed on the system
- [ ] Implement a server that return the list of files in a directory
- [ ] *Implement a server that return the list of Bluetooth devices and connect/disconnect to them

* Maybe

### DX

- [ ] Implement a way to run the client and servers in the background for testing
- [ ] Implement a integration test suite

## Development Environment

There are 3 main process in this project: UI(main), Client, and Server. The UI is responsible for rendering the user interface, the client is responsible for forwarding the user's input to the server processes, and the server processes are responsible for processing the user's input and returning a list of items.

They log to log files in /tmp/{name}.log directory. When working on the project, you can run the following commands to see the logs:

Recommend running `tail` in a separate terminal window to see the logs in real-time.

```bash
# First terminal
tail -f /tmp/ui.log

# Second terminal
tail -f /tmp/client.log

# Third terminal
tail -f /tmp/server1.log
```

Then in another terminal, you can run the following commands to start the processes:

```bash
make run
```
