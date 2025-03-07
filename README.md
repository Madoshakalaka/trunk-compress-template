
# How to run

```bash
trunk build 

cargo r -p backend 8000
```

with logs

```bash
RUST_LOG=debug cargo r -p backend 8000
```

with compression

```bash
cargo r -p backend -F compression 8000 
```
