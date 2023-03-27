`main.rs` handles running the program
`lib.rs` handles all the logic of the task at hand

### Usage
```
cargo run -- Some peom.txt
```

### Seperation of concerns for Binary projects
- split the program into `lib.rs` and `main.rs`, where `lib.rs` has the programs logic
- `main.rs` should be limited to orchestration and running the program 

### Case sensitivity
Add `IGNORE_CASE` env variable 
