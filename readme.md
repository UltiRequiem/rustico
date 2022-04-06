# rustico

![Para divertirme](https://raw.githubusercontent.com/UltiRequiem/rustico/e9e5c1c50f566f1946d9e506bb501b2e78cb551c/para_divertirme.png)

> [Doing it for the fun.](https://youtu.be/v3rOpmlpFsM)

Aren't you _cansado_ from writting Rust Programs in English? Do you like saying
_"chales"_ a lot? Would you like to try something different, in an exotic and
funny-sounding language?

**rustico** (Spanish for Rust) is here to save your day, as it allows you to
write Rust programs in Spanish.

You are from Costa Rica and don't feel as ease using only Spanish Words? Don't
Worry! Rustico if fully compatible English-Rust, you can mix both at your
convenience.

## instalacion

```sh
cargo install rustico
```

## uso

Here 's an exmple of what can be achieved with Rustico.

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

        chales!("f")
    }
}
```

Check [`examples/`](./examples) for more.

## documentacion

For now you can see [`rustico/src/lib.rs`](./rustico/src/lib.rs) for all the
vocabulary.

## contribuyendo

First of all, _Piolisimo_ that you want to participate in this joke! To add
words is just neccesary to modify [`rustico/src/lib.rs`](./rustico/src/lib.rs).

## por que?

- If the French can do it, so can we

- los hahas

## gracias

[Benjamin Bouvier](https://github.com/bnjbvr) for the original French
implementation.

## licencia

Licensed under the MIT License 📄
