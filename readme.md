# The Ray Tracer Challenge
Following along with [The Ray Tracer Challenge](https://pragprog.com/book/jbtracer/the-ray-tracer-challenge) Book
I don't know anything about ray tracers or graphics, I barely know anything
about Rust, seems fun to learn about them though.


## Notes
- Left handed coordinates
- Epsilon in the book is `0.0001` only works with `f32`, use `std::f64::EPSILON`
- [Points vs Vectors](https://math.stackexchange.com/questions/645672/what-is-the-difference-between-a-point-and-a-vector)
  - Point + Vector = Point
  - Vector + Point = Point
  - Vector + Vector = Vector
  - Point + Point = NOPE, not a real thing
  - Point - Vector = Point
  - Point - Point = Vector
  - Vector - Vector = Vector
  - Vector - Point = NOPE, not a real thing
