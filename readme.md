# Cargo build

---
to initalise cargo project in a given directory  

```
cargo init
```
---
Cargo.toml file is similar to the package.json file

---
to run the project execute `cargo run` command

OR  

.target/debug/cargo (cargo is the name of the executable)

---

`cargo build` : build the project but does not run it, not optimized for prod
`cargo build --release` : for production build, optimized for prod, creates a release folder and creates binary (executable `cargo` file in this case)
