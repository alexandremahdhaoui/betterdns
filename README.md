# betterdns

We would like to register new vms to the DNS as soon as they're up & running. 
To randomly assign any IPs to new vms/container or use DHCP.

The goal of this project is to achieve **decentralized server-side service discovery**.

Issues: 
- How do we unregister vms/containers that might shut down in a non-gracefully manner?
  - Use health checks.
  - Try to terminate non-healthy vms/containers.
  - Then unregister them from the DNS.

## Repository structure

| Path                      | Description                    |
|---------------------------|--------------------------------|
| `src/dns_controller`      | Controller for the dns server. |
| `src/server`              | REST API server.               |
| `src/dns_zonefile_parser` | Parser for dns zonefile.       |

## TODO: Next steps
- [ ] Write tests.

## Getting started

This binary will create a REST API server to {create,read,update,delete} records for the nameserver.

| Endpoint    | Method | Description                   |
|-------------|--------|-------------------------------|
| `/a`        | GET    | get all `A record`s.          |
| `/a/<name>` | GET    | get one `A record` by name.   |
| `/a`        | POST   | create a new `A record`.      |
| `/a/<name>` | PUT    | update an `A record` by name. |
| `/a/<name>` | DELETE | delete an` A record` by name. |
| `/`         | GET    | get the whole `dns` manifest  |

At the beginning we will use this as a monorepo. And will contain other business logic:
- Spawn a `coredns` process.
- Listen to change on `dns_file`.
  - https://docs.rs/notify/latest/notify/
  - https://docs.rs/notify/4.0.15/notify/enum.DebouncedEvent.html 
- When change happen, restart `coredns` process.

## Installation

```shell
git clone https://gitlab.com/alexandre.mahdhaoui/betterdns && cd betterdns
cargo build --release
target/release/betterdns
```

## Considerations

We will have a problem when we will try to authenticate/authorize clients.
Indeed, we'll maybe try to use JWT tokens to authenticate new VM/container trying to register to the DNS.