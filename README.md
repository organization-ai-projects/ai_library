# ai_library

> Librairie AI minimaliste, modulaire et 100% Rust pour usage partagé.
>
> - Backend CPU natif (pas ndarray, pas torch, pur Vec<f32>).
> - Tensor ND minimal (à venir).
> - Hooks pour CUDA, autograd, graph, etc. (prévu).
> - API extensible, open-source, "torch-like" mais full maison.

## Exemple

```rust
use ai_library::hello_ai;

fn main() {
    println!("{}", hello_ai());
}
```
