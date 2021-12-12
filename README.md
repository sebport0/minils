An imitation of ls inspired on minigrep from [An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).

## How to use

Go to the root directory of the project. 

### Show help

```
cargo run -- --help
```

### List directory contents

```
cargo run .
```

### List /a/dir/ contents

```
cargo run /a/dir
```

### Show additional information

To show permissions, size(in bytes), creation time, last modified time and last accessed times use
the list flag:

```
cargo run /a/dir --list
```