Enable ip forwarding:

```
sysctl -w net.ipv4.ip_forward=1
sysctl -w net.ipv6.conf.all.forwarding=1
```

```
cargo run
```

```
ngrok http 1234
```
