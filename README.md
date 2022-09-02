# safe_index

This project implements type-safe indexation. It provides just one macro `init` that:
- Generates a wrapper over `usize`, used as index, from the identifier specified at the first argument. 
This wrapper implements some basic traits such as `Copy`, `Clone`, `Eq`, etc.
- Generates a wrapper over `Vec` that uses type mentionned before for its indexation purpose. It means that you can use `container[index]` syntax. 
This wrapper implements some basic operaions over vector  like **add**, **remove**, **iter** and **into_iter**.
- Generates a set that allows to store index keys.
