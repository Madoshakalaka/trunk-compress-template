
# How to run

```bash
trunk build 

cargo r -p backend 8000
```

with logs:

```bash
RUST_LOG=debug cargo r -p backend -F env-filter 8000
```

with compression (make sure [trunk-compress](https://github.com/Madoshakalaka/trunk-compress/releases/latest) is in your PATH)

```bash
cargo r -p backend -F compression 8000 
```

With journald logging:

```bash
cargo build --release -F compression,journald
```

in your systemd service file:

```service
[Service]
ExecStart=/path/to/your/backend 80

