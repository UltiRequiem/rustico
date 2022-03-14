# rostico

![Para divertirme](./para_divertirme.png)

```sh
cargo install rustico
```

## Usage

```rust
rustico::rustico! {
    funcion principal() {
        deja mutable x = 31;

        machea x {
            42 => {
                chales!("Like panic")
            }
            _ => imprime!("Buenas!")
        }

        para numero de 0..10 {
          imprime!(numero)
        }
    }
}
```

Check [examples/](./examples) for more.

## Docs

Check [`rustico/src/lib.rs`](./rustico/src/lib.rs) for all the vocabulary.

## La licencia

Licensed under the MIT License.
