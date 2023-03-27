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


### Standard output and Standard error
Command line programs are expected to send error messages to the standard error stream so we can still see error 
messages on the screen even if we redirect the standard output stream to a file.

Run your program with `cargo run -- to poem.txt > output.txt`, this will create the `output.txt` file with the standard output data