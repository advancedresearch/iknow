# Iknow
A self-describing knowledge format with support for Rust-like syntax

### Motivation

Bootstrap knowledge format from a root knowledge format
that is capable of describing other knowledge formats.

### Self-Description of Root Knowledge Structure

The following is both valid Rust code and Iknow format.
It describes the internal representation of the Iknow format in Rust.

"assets/self_root.txt":
```text
enum Root {
    Ty(Arc<String>),
    Str(Arc<String>),
    F64(f64),
    Bool(bool),
    Avatar(Box<(Self, Self)>),
    Tup(Vec<Self>),
    Struct {name: Box<Self>, fields: Vec<Self>},
    Enum {name: Box<Self>, variants: Vec<Self>},
    Instance {class: usize, data: Option<Box<Self>>},
    InstanceTy {ty: Box<Self>, data: Option<Box<Self>>},
}
```

### Generic Version

The root knowledge format can describe a generic version of itself.
This is not valid Rust code, due to a limitation
in how to annotate a custom/generic type using `.` before the name, e.g. `.T`.

"source/test/generics/gen_root.txt":
```text
enum Root<.T> {
    Ty(Arc<String>),
    Val(.T),
    Avatar(Box<(Self<.T>, Self<.T>)>),
    Tup(Vec<Self<.T>>),
    Struct {name: Box<Self<.T>>, fields: Vec<Self<.T>>},
    Enum {name: Box<Self<.T>>, variants: Vec<Self<.T>>},
    Instance {class: usize, data: Option<Box<Self<.T>>>},
    InstanceTy {ty: Box<Self<.T>>, data: Option<Box<Self<.T>>>},
}
```

### Origin of name "Iknow"

In the Star Wars V movie, Han Solo is frozen while Leia is watching.

Leia: "I love you"

Han: "I know"
