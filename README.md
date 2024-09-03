# search_code

A Toy CLI Tool to search code line .
Honestly, the error process is not good, and the code isn't clean. Just a toy tool for practicing rust.

**Usage**

- This search the symbol `main` in the file `./src/main.rs` with language `rust` and set search_symbol to `true`.

```bash
search_code -k "main" -p ./src/main.rs -l rust -s true 
```

- Also, if just provide a directory, it will search all files in the directory.

```bash
search_code -k "main" -p ./src -l rust -s true 
```

- If set `-s` to `false`, just search `key` in the file or the directory.

```bash
search_code -k "main" -p ./src -l rust -s false 
```

