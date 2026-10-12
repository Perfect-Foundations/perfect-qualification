# Independent compile-negative dimension and affine-point consumer

These are actual Rust compiler invocations through rustdoc.

```compile_fail
use perfect_units::{Quantity,dimensions::{Length,Time}};
use perfect_rational::Rational;
let metres=Quantity::<Rational,Length>::new_canonical(Rational::from(5));
let seconds=Quantity::<Rational,Time>::new_canonical(Rational::from(10));
let _=metres+seconds;
```

```compile_fail
use perfect_units::{QuantityPoint,dimensions::Temperature};
use perfect_rational::Rational;
let a=QuantityPoint::<Rational,Temperature>::new_canonical(Rational::from(3));
let b=QuantityPoint::<Rational,Temperature>::new_canonical(Rational::from(4));
let _=a+b;
```

```compile_fail
use perfect_units::{Quantity,QuantityPoint,dimensions::Temperature};
use perfect_rational::Rational;
let point=QuantityPoint::<Rational,Temperature>::new_canonical(Rational::from(3));
let delta=Quantity::<Rational,Temperature>::new_canonical(Rational::from(4));
let _=point*delta;
```
