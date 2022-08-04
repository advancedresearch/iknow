use std::sync::Arc;
use std::fmt;

pub mod parsing;

#[derive(Debug, PartialEq)]
pub enum Root {
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

impl Root {
    pub fn ty_self() -> Root {
        Root::Ty(Arc::new("Self".into()))
    }

    pub fn ty_arc() -> Root {
        Root::Ty(Arc::new("Arc".into()))
    }

    pub fn ty_string() -> Root {
        Root::Ty(Arc::new("String".into()))
    }

    pub fn ty_f64() -> Root {
        Root::Ty(Arc::new("f64".into()))
    }

    pub fn ty_bool() -> Root {
        Root::Ty(Arc::new("bool".into()))
    }

    pub fn ty_box() -> Root {
        Root::Ty(Arc::new("box".into()))
    }

    pub fn ty_usize() -> Root {
        Root::Ty(Arc::new("usize".into()))
    }

    pub fn ty_option() -> Root {
        Root::Ty(Arc::new("Option".into()))
    }

    pub fn ty_vec() -> Root {
        Root::Ty(Arc::new("Vec".into()))
    }
}

pub fn root_self() -> Root {
    use Root::*;

    Enum {
        name: Box::new(Str(Arc::new("Root".into()))),
        variants: vec![
            Avatar(Box::new((
                Str(Arc::new("Ty".into())),
                Avatar(Box::new((
                    Root::ty_arc(),
                    Root::ty_string(),
                ))),
            ))),
            Avatar(Box::new((
                Str(Arc::new("Str".into())),
                Avatar(Box::new((
                    Root::ty_arc(),
                    Root::ty_string(),
                ))),
            ))),
            Avatar(Box::new((
                Str(Arc::new("F64".into())),
                Root::ty_f64(),
            ))),
            Avatar(Box::new((
                Str(Arc::new("Bool".into())),
                Root::ty_bool(),
            ))),
            Avatar(Box::new((
                Str(Arc::new("Avatar".into())),
                Avatar(Box::new((
                    Root::ty_box(),
                    Tup(vec![
                        Root::ty_self(),
                        Root::ty_self(),
                    ]),
                ))),
            ))),
            Avatar(Box::new((
                Str(Arc::new("Tup".into())),
                Avatar(Box::new((
                    Root::ty_vec(),
                    Root::ty_self(),
                ))),
            ))),
            Struct {
                name: Box::new(Str(Arc::new("Struct".into()))),
                fields: vec![
                    Tup(vec![
                        Str(Arc::new("name".into())),
                        Avatar(Box::new((
                            Root::ty_box(),
                            Root::ty_self(),
                        ))),
                    ]),
                    Tup(vec![
                        Str(Arc::new("fields".into())),
                        Avatar(Box::new((
                            Root::ty_vec(),
                            Root::ty_self(),
                        ))),
                    ]),
                ],
            },
            Struct {
                name: Box::new(Str(Arc::new("Enum".into()))),
                fields: vec![
                    Tup(vec![
                        Str(Arc::new("name".into())),
                        Avatar(Box::new((
                            Root::ty_box(),
                            Root::ty_self(),
                        ))),
                    ]),
                    Tup(vec![
                        Str(Arc::new("variants".into())),
                        Avatar(Box::new((
                            Root::ty_vec(),
                            Root::ty_self(),
                        ))),
                    ]),
                ],
            },
            Struct {
                name: Box::new(Str(Arc::new("Instance".into()))),
                fields: vec![
                    Tup(vec![
                        Str(Arc::new("class".into())),
                        Root::ty_usize(),
                    ]),
                    Tup(vec![
                        Str(Arc::new("data".into())),
                        Avatar(Box::new((
                            Root::ty_option(),
                            Avatar(Box::new((
                                Root::ty_box(),
                                Root::ty_self(),
                            ))),
                        ))),
                    ]),
                ],
            },
            // InstanceTy {ty: Box<Self>, data: Option<Box<Self>>},
            Struct {
                name: Box::new(Str(Arc::new("InstanceTy".into()))),
                fields: vec![
                    Tup(vec![
                        Str(Arc::new("ty".into())),
                        Avatar(Box::new((
                            Root::ty_box(),
                            Root::ty_self(),
                        ))),
                    ]),
                    Tup(vec![
                        Str(Arc::new("data".into())),
                        Avatar(Box::new((
                            Root::ty_option(),
                            Avatar(Box::new((
                                Root::ty_box(),
                                Root::ty_self(),
                            ))),
                        ))),
                    ]),
                ]
            }
        ]
    }
}

impl fmt::Display for Root {
    fn fmt(&self, w: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Root::*;

        match self {
            Ty(ty) => write!(w, "(ty {:?})", ty)?,
            Str(s) => write!(w, "{:?}", s)?,
            F64(v) => write!(w, "{}", v)?,
            Bool(b) => write!(w, "{}", b)?,
            Avatar(ab) => write!(w, "(ava {} {})", ab.0, ab.1)?,
            Tup(v) => {
                write!(w, "(tup ")?;
                for i in v {
                    write!(w, "{} ", i)?;
                }
                write!(w, ")")?;
            }
            Struct {name, fields} => {
                write!(w, "struct {} {{", name)?;
                for f in fields {
                    write!(w, "{}, ", f)?;
                }
                write!(w, "}}")?;
            }
            Enum {name, variants} => {
                write!(w, "enum {} {{", name)?;
                for f in variants {
                    write!(w, "{}, ", f)?;
                }
                write!(w, "}}")?;
            }
            Instance {class, data} => if let Some(data) = data {
                write!(w, "(ins {} {})", class, data)?
            } else {
                write!(w, "(ins {})", class)?
            }
            InstanceTy {ty, data} => if let Some(data) = data {
                write!(w, "(ins {} {})", ty, data)?
            } else {
                write!(w, "(ins {})", ty)?
            }
        }
        Ok(())
    }
}

/// Joins format with data into a single structure.
pub fn join_format_data(format: &str, data: &str) -> Result<String, String> {
    let format = parsing::parse_str(format, &[])?;
    let data = parsing::parse_str(data, &[])?;

    let mut res = vec![];
    res.push(format);
    if let Root::Tup(records) = data {
        for rec in records {
            res.push(Root::Instance {class: 0, data: Some(Box::new(rec))});
        }
    }
    Ok(format!("{}", Root::Tup(res)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self() {
        let a = match parsing::parse_str(include_str!("../assets/self_root.txt"), &[]) {
            Ok(x) => x,
            Err(err) => panic!("ERROR:\n{}", err),
        };
        assert_eq!(a, root_self());
    }

    #[test]
    fn test_person() {
        let format_with_data = match parsing::parse_str(
            include_str!("../source/test/person/person-with-instance.txt"), &[]) {
            Ok(x) => x,
            Err(err) => panic!("ERROR:\n{}", err),
        };
        assert_eq!(join_format_data(
            include_str!("../source/test/person/person.txt"),
            include_str!("../source/test/person/table-data.txt")
        ).unwrap(), format!("{}", format_with_data));

        let _meta = match parsing::parse_str(
            include_str!("../source/test/person/meta.txt"), &[]) {
            Ok(x) => x,
            Err(err) => panic!("ERROR:\n{}", err),
        };
    }

    #[test]
    fn test_graph() {
        use Root::*;

        let _edge_dir = match parsing::parse_str(
            include_str!("../source/test/graph/edge_dir.txt"), &[]) {
            Ok(x) => x,
            Err(err) => panic!("ERROR:\n{}", err),
        };
        let _edge = match parsing::parse_str(
            include_str!("../source/test/graph/edge.txt"), &[]) {
            Ok(x) => x,
            Err(err) => panic!("ERROR:\n{}", err),
        };
        let _node = match parsing::parse_str(
            include_str!("../source/test/graph/node.txt"), &[]) {
            Ok(x) => x,
            Err(err) => panic!("ERROR:\n{}", err),
        };
        let _node_data = match parsing::parse_str(
            include_str!("../source/test/graph/node-data.txt"), &[]) {
            Ok(x) => x,
            Err(err) => panic!("ERROR:\n{}", err),
        };
        let edge_data = match parsing::parse_str(
            include_str!("../source/test/graph/edge-data.txt"), &[]) {
            Ok(x) => x,
            Err(err) => panic!("ERROR:\n{}", err),
        };
        assert_eq!(
            edge_data,
            Tup(vec![
                Tup(vec![F64(0.0), F64(1.0), Str(Arc::new("Left".into()))]),
            ])
        );
        let edge_data2 = match parsing::parse_str(
            include_str!("../source/test/graph/edge-data2.txt"), &[]) {
            Ok(x) => x,
            Err(err) => panic!("ERROR:\n{}", err),
        };
        assert_eq!(
            edge_data2,
            Tup(vec![
                Tup(vec![F64(0.0), F64(1.0),
                    InstanceTy {
                        ty: Box::new(Ty(Arc::new("EdgeDir".into()))),
                        data: Some(Box::new(Str(Arc::new("Left".into())))),
                    },
                ]),
            ])
        );
        let edge_data3 = match parsing::parse_str(
            include_str!("../source/test/graph/edge-data3.txt"), &[]) {
            Ok(x) => x,
            Err(err) => panic!("ERROR:\n{}", err),
        };
        assert_eq!(
            edge_data3,
            Tup(vec![
                Tup(vec![F64(0.0), F64(1.0),
                    InstanceTy {
                        ty: Box::new(Ty(Arc::new("EdgeDir".into()))),
                        data: Some(Box::new(Str(Arc::new("Left".into())))),
                    },
                ]),
            ])
        );
    }

    #[test]
    fn test_generics() {
        let _gen_root = match parsing::parse_str(
            include_str!("../source/test/generics/gen_root.txt"), &[]) {
            Ok(x) => x,
            Err(err) => panic!("ERROR:\n{}", err),
        };
        let _gen_struct = match parsing::parse_str(
            include_str!("../source/test/generics/gen_struct.txt"), &[]) {
            Ok(x) => x,
            Err(err) => panic!("ERROR:\n{}", err),
        };
    }
}
