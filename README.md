## Simple worker/consumer using amqp
This is one part of my own tech stack for personal projects, this tech stech is composed of:

### Design System
Design system Maintain cohesion of design/ui through my projects and redy to use components to use in any projects. Any new component that is needed will also be added to the design system for tuture use.

### Server
Server Basic integration of a server without business rules and processes, mostly sync processing and db integration. It will handle basic auth and observability 
out of the box, i am assuming every project will need at least that. If i see the need along the way the improvements will be implemented in the core server if it is not 
a business rule, that way i can guarantee it is evolving but not holding complexity based on product, remaining a core implementation for generic use.

### Worker/Consumer

Simple implemention of a worker/consumer that will be used to process assincronous tasks, usually tasks that take more resources and can be deferred to a worker. 
It will have basic security and observability implementations out of the box. It will use rabbitmq as message broker. Will be improved over time from the same concept of core service.


### Usefull commands

Start rabbitmq using docker locally:
```bash
docker compose up -d
```
Compile the code
```bash
cargo build
```
Run the code
```bash
cargo run (run the code)
```

The broker will start at `http://localhost:15672/`

Login using the credentials 
```bash
login: guest
pssword: guest
```

Find the queue `queue_test` and publish any message, 
the content will be printed on the terminal.

### Links for the other repos in the tech stack

- [Design System](https://github.com/caiocampoos/mapinguatech-design-system)
- [Server]() `soon`
