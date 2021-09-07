# Bindings for SAM3x8e

Device support bindings for Atmel SAM3x8e, generated using `svd2rust`

```
cargo init --lib
svd2rust -i ./ATSAM3X8E.svd
form -i lib.rs -o ./src
rm lib.rs
cargo fmt
```