<!-- cargo-rdme start -->

A nice and configurable derive macro for getters & setters. See derive macro
docs for full list of options

[![MASTER CI status](https://github.com/Alorel/accessory-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Alorel/accessory-rs/actions/workflows/ci.yml?query=branch%3Amaster)
[![crates.io badge](https://img.shields.io/crates/v/accessory)](https://crates.io/crates/accessory)
[![docs.rs badge](https://img.shields.io/docsrs/accessory?label=docs.rs)](https://docs.rs/accessory)
[![dependencies badge](https://img.shields.io/librariesio/release/cargo/accessory)](https://libraries.io/cargo/accessory)

# Examples

## Basic usage

```rust
#[derive(Default, accessory::Accessors)]
struct Structopher {
  /// The comment gets copied over
  #[access(set, get, get_mut)] // Generate a setter, getter ant mut getter
  field: String,
  _field2: u8, // Generate nothing
}
let mut data = Structopher::default();
data.set_field("Hello, world!".to_string());

let get: &String = data.field();
assert_eq!(get, "Hello, world!", "get(1)");

let mut get: &mut String = data.field_mut();
*get = "Hello, universe!".to_string();

let mut get = data.field();
assert_eq!(get, "Hello, universe!", "get(2)");
```

### Generated output

```rust
impl Structopher {
    #[doc = " The comment gets copied over"]
    #[inline]
    pub fn field(&self) -> &String { &self.field }
    #[doc = " The comment gets copied over"]
    #[inline]
    pub fn field_mut(&mut self) -> &mut String { &mut self.field }
    #[doc = " The comment gets copied over"]
    #[inline]
    pub fn set_field(&mut self, new_value: String) -> &mut Self {
        self.field = new_value;
        self
    }
}
````

## Option inheritance

Option priority is as follows:

1. Field attribute
   1. Per-accessor type (`get`, `get_mut`, `set`)
   1. Catch-all (`all`)
1. Container attribute (`defaults`)
   1. Per-accessor type (`get`, `get_mut`, `set`)
   1. Catch-all (`all`)

```rust
#[derive(accessory::Accessors, Default, Eq, PartialEq, Debug)]
#[access(
  get, set, // derive these for all fields by default
  // set defaults for whenever
  defaults(
    all(
      const_fn, // Make it a const fn
      owned, // use `self` and not `&self`
      cp // Treat it as a copy type. Treats it as a reference if not set & not `owned`
    ),
    get(
      owned = false, // overwrite from `all`
      vis(pub(crate)) // set visibilty to `pub(crate)`
    )
  )
)]
struct Structopher {
    #[access(
      all(const_fn = false), // Disable the container's const_fn for this field
      get(const_fn),  // But re-enable it for the getter
      get_mut // enable with defaults
    )]
    x: i8,
    y: i8,

    #[access(get_mut(skip))] // skip only get_mut
    z: i8,

    #[access(skip)] // skip this field altogether
    w: i8,
}

const INST: Structopher = Structopher { x: 0, y: 0, z: 0, w: 0 }
  .set_y(-10)
  .set_z(10);

let mut inst = Structopher::default();
inst = inst.set_x(10);
*inst.x_mut() += 1;

assert_eq!(INST, Structopher { x: 0, y: -10, z: 10, w: 0 } , "const instance");
assert_eq!(inst, Structopher { x: 11, y: 0, z: 0, w: 0 } , "instance");
```

### Generated output

```rust
impl Structopher {
    #[inline]
    pub(crate) const fn x(&self) -> i8 { self.x }
    #[inline]
    pub fn x_mut(mut self) -> i8 { self.x }
    #[inline]
    pub fn set_x(mut self, new_value: i8) -> Self {
        self.x = new_value;
        self
    }
    #[inline]
    pub(crate) const fn y(&self) -> i8 { self.y }
    #[inline]
    pub const fn set_y(mut self, new_value: i8) -> Self {
        self.y = new_value;
        self
    }
    #[inline]
    pub(crate) const fn z(&self) -> i8 { self.z }
    #[inline]
    pub const fn set_z(mut self, new_value: i8) -> Self {
        self.z = new_value;
        self
    }
}
````

## Names & types

You can modify function return types & names

```rust
#[derive(Default, accessory::Accessors)]
#[access(defaults(get(prefix(get))))]
struct Structopher {
    #[access(
      get(suffix(right_now), ty(&str)), // set the suffix and type
      get_mut(suffix("")) // remove the inherited suffix set by `get_mut`
    )]
    good: String,
}
let mut inst = Structopher::default();
*inst.good() = "On it, chief".into();
assert_eq!(inst.get_good_right_now(), "On it, chief");
```

### Generated output

```rust
impl Structopher {
    #[inline]
    pub fn get_good_right_now(&self) -> &str { &self.good }
    #[inline]
    pub fn good(&mut self) -> &mut String { &mut self.good }
}
````

<!-- cargo-rdme end -->
