# betterdns

We would like to register new vms to the DNS as soon as they're up & running. 
To randomly assign any IPs to new vms/container or use DHCP.

The goal of this project is to achieve **service discovery**.

Issues: 
- How do we unregister vms/containers that might shut down in a non-gracefully manner?
  - Use health checks.
  - Try to terminate non-healthy vms/containers.
  - Then unregister them from the DNS.

## Repository structure

| Path                      | Description                                                                                                              |
|---------------------------|--------------------------------------------------------------------------------------------------------------------------|
| `src/dns_manifest_parser` | Parser for dns zone manifest.                                                                                            |
| `src/dns_operator`        | Operator of the dns server. Highly decoupled into 3 internal services:<br/>`controller.rs`, `runtime.rs` & `watcher.rs`. |
| `src/rest_api`            | REST API server.                                                                                                         |

## TODO: Next steps
- [ ] Write tests.

## Getting started

This binary will create 2 threads:
- One thread serving the REST API to {create,read,update,delete} records of the dns zone manifest.
- Another one will act as an operator, watching the dns zone manifest for change & updating the nameserver.
  - The operator is responsible for running the nameserver as a child process.

| Endpoint    | Method | Description                   |
|-------------|--------|-------------------------------|
| `/`         | GET    | get the whole `dns` manifest  |
| `/a`        | GET    | get all `A record`s.          |
| `/a/<name>` | GET    | get one `A record` by name.   |
| `/a`        | POST   | create a new `A record`.      |
| `/a/<name>` | PUT    | update an `A record` by name. |
| `/a/<name>` | DELETE | delete an` A record` by name. |

## Installation

```shell
git clone https://gitlab.com/alexandre.mahdhaoui/betterdns
mv betterdns "$HOME"
"$HOME/betterdns/scripts/update.sh"
```

Please make sure to have a customized copy of `Corefile`, `dns_manifest` & `Rocket.toml` in the directory that runs `betterdns`.

If you want to secure your alpine with a firewall:
- https://wiki.alpinelinux.org/wiki/How-To_Alpine_Wall

## Considerations

We will have a problem when we will try to authenticate/authorize clients.
Indeed, we'll maybe try to use JWT tokens to authenticate new VM/container trying to register to the DNS.
