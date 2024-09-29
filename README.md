# easings
a simple rust crate for easing functions

[documentation](https://wcaleniekubaa.github.io/easings/)

## instalation

im not publishing that crate to crates.io for now, i need a name since easings is taken
```bash
cargo add easings --git https://github.com/wcaleniekubaa/easings
```

## usage

```rust
fn main() {
    let y = easings::linear(0.5);
    // y = 0.5 

    let y = easings::InSine::get(0.5);
    // y = 0.2928932188134524 

    let y = easings::Type::OutSine.get(0.5);
    // y = 0.7071067811865476 
}

fn lerp<E: easings::Easing>(a: f64, b: f64, t: f64) -> f64 {
    let y = E::get(t);
    a + (b - a) * y
}


fn lerp2(a: f64, b: f64, t: f64, ty: easings::Type) -> f64 {
    let y = ty.get(t);
    a + (b - a) * y
}
```


