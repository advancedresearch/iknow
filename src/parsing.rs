use crate::*;

use piston_meta::{Convert, Range};

fn parse_expr(node: &str, dirs: &[String], mut convert: Convert, ignored: &mut Vec<Range>) -> Result<(Range, Root), ()> {
    let start = convert;
    let start_range = convert.start_node(node)?;
    convert.update(start_range);

    let mut expr: Option<Root> = None;
    loop {
        if let Ok(range) = convert.end_node(node) {
            convert.update(range);
            break;
        } else if let Ok((range, val)) = parse_enum(dirs, convert, ignored) {
            convert.update(range);
            expr = Some(val);
        } else if let Ok((range, val)) = parse_struct(dirs, convert, ignored) {
            convert.update(range);
            expr = Some(val);
        } else if let Ok((range, val)) = parse_tup(dirs, convert, ignored) {
            convert.update(range);
            expr = Some(val);
        } else if let Ok((range, val)) = parse_ava(dirs, convert, ignored) {
            convert.update(range);
            expr = Some(val);
        } else if let Ok((range, val)) = parse_ty(dirs, convert, ignored) {
            convert.update(range);
            expr = Some(val);
        } else if let Ok((range, val)) = parse_ins(dirs, convert, ignored) {
            convert.update(range);
            expr = Some(val);
        } else if let Ok((range, val)) = parse_enum_var(dirs, convert, ignored) {
            convert.update(range);
            expr = Some(val);
        } else if let Ok((range, _)) = convert.meta_bool("self") {
            convert.update(range);
            expr = Some(Root::ty_self());
        } else if let Ok((range, _)) = convert.meta_bool("string") {
            convert.update(range);
            expr = Some(Root::ty_string());
        } else if let Ok((range, _)) = convert.meta_bool("usize") {
            convert.update(range);
            expr = Some(Root::ty_usize());
        } else if let Ok((range, _)) = convert.meta_bool("f64") {
            convert.update(range);
            expr = Some(Root::ty_f64());
        } else if let Ok((range, _)) = convert.meta_bool("bool") {
            convert.update(range);
            expr = Some(Root::ty_bool());
        } else if let Ok((range, _)) = convert.meta_bool("arc") {
            convert.update(range);
            expr = Some(Root::ty_arc());
        } else if let Ok((range, _)) = convert.meta_bool("box") {
            convert.update(range);
            expr = Some(Root::ty_box());
        } else if let Ok((range, _)) = convert.meta_bool("opt") {
            convert.update(range);
            expr = Some(Root::ty_option());
        } else if let Ok((range, _)) = convert.meta_bool("vec") {
            convert.update(range);
            expr = Some(Root::ty_vec());
        } else if let Ok((range, val)) = convert.meta_string("str") {
            convert.update(range);
            expr = Some(Root::Str(val));
        } else if let Ok((range, val)) = convert.meta_f64("num") {
            convert.update(range);
            expr = Some(Root::F64(val));
        } else {
            let range = convert.ignore();
            convert.update(range);
            ignored.push(range);
        }
    }

    let expr = expr.ok_or(())?;
    Ok((convert.subtract(start), expr))
}

fn parse_enum(dirs: &[String], mut convert: Convert, ignored: &mut Vec<Range>) -> Result<(Range, Root), ()> {
    let node = "enum";

    let start = convert;
    let start_range = convert.start_node(node)?;
    convert.update(start_range);

    let mut name: Option<Box<Root>> = None;
    let mut variants: Vec<Root> = vec![];
    loop {
        if let Ok(range) = convert.end_node(node) {
            convert.update(range);
            break;
        } else if let Ok((range, val)) = convert.meta_string("name") {
            convert.update(range);
            name = Some(Box::new(Root::Str(val)));
        } else if let Ok((range, val)) = parse_ava(dirs, convert, ignored) {
            convert.update(range);
            name = Some(Box::new(val));
        } else if let Ok((range, val)) = parse_expr("variant", dirs, convert, ignored) {
            convert.update(range);
            variants.push(val);
        } else if let Ok((range, val)) = convert.meta_string("item") {
            convert.update(range);
            variants.push(Root::Str(val));
        } else {
            let range = convert.ignore();
            convert.update(range);
            ignored.push(range);
        }
    }

    let name = name.ok_or(())?;
    Ok((convert.subtract(start), Root::Enum {name, variants}))
}

fn parse_struct(dirs: &[String], mut convert: Convert, ignored: &mut Vec<Range>) -> Result<(Range, Root), ()> {
    let node = "struct";

    let start = convert;
    let start_range = convert.start_node(node)?;
    convert.update(start_range);

    let mut name: Option<Box<Root>> = None;
    let mut fields: Vec<Root> = vec![];
    loop {
        if let Ok(range) = convert.end_node(node) {
            convert.update(range);
            break;
        } else if let Ok((range, val)) = convert.meta_string("name") {
            convert.update(range);
            name = Some(Box::new(Root::Str(val)));
        } else if let Ok((range, val)) = parse_ava(dirs, convert, ignored) {
            convert.update(range);
            name = Some(Box::new(val));
        } else if let Ok((range, val)) = parse_expr("field", dirs, convert, ignored) {
            convert.update(range);
            fields.push(val);
        } else {
            let range = convert.ignore();
            convert.update(range);
            ignored.push(range);
        }
    }

    let name = name.ok_or(())?;
    Ok((convert.subtract(start), Root::Struct {name, fields}))
}

fn parse_tup(dirs: &[String], mut convert: Convert, ignored: &mut Vec<Range>) -> Result<(Range, Root), ()> {
    let node = "tup";

    let start = convert;
    let start_range = convert.start_node(node)?;
    convert.update(start_range);

    let mut items: Vec<Root> = vec![];
    loop {
        if let Ok(range) = convert.end_node(node) {
            convert.update(range);
            break;
        } else if let Ok((range, val)) = parse_expr("item", dirs, convert, ignored) {
            convert.update(range);
            items.push(val);
        } else if let Ok((range, val)) = convert.meta_string("item") {
            convert.update(range);
            items.push(Root::Str(val));
        } else {
            let range = convert.ignore();
            convert.update(range);
            ignored.push(range);
        }
    }

    Ok((convert.subtract(start), Root::Tup(items)))
}

fn parse_ava(dirs: &[String], mut convert: Convert, ignored: &mut Vec<Range>) -> Result<(Range, Root), ()> {
    let node = "ava";

    let start = convert;
    let start_range = convert.start_node(node)?;
    convert.update(start_range);

    let mut a: Option<Root> = None;
    let mut b: Option<Root> = None;
    loop {
        if let Ok(range) = convert.end_node(node) {
            convert.update(range);
            break;
        } else if let Ok((range, val)) = convert.meta_string("a") {
            convert.update(range);
            a = Some(Root::Str(val));
        } else if let Ok((range, val)) = parse_expr("a", dirs, convert, ignored) {
            convert.update(range);
            a = Some(val);
        } else if let Ok((range, val)) = parse_expr("b", dirs, convert, ignored) {
            convert.update(range);
            b = Some(val);
        } else {
            let range = convert.ignore();
            convert.update(range);
            ignored.push(range);
        }
    }

    let a = a.ok_or(())?;
    let b = b.ok_or(())?;
    Ok((convert.subtract(start), Root::Avatar(Box::new((a, b)))))
}

fn parse_ty(_dirs: &[String], mut convert: Convert, ignored: &mut Vec<Range>) -> Result<(Range, Root), ()> {
    let node = "ty";

    let start = convert;
    let start_range = convert.start_node(node)?;
    convert.update(start_range);

    let mut name: Option<Arc<String>> = None;
    loop {
        if let Ok(range) = convert.end_node(node) {
            convert.update(range);
            break;
        } else if let Ok((range, val)) = convert.meta_string("name") {
            convert.update(range);
            name = Some(val);
        } else {
            let range = convert.ignore();
            convert.update(range);
            ignored.push(range);
        }
    }

    let name = name.ok_or(())?;
    Ok((convert.subtract(start), Root::Ty(name)))
}

fn parse_enum_var(dirs: &[String], mut convert: Convert, ignored: &mut Vec<Range>) -> Result<(Range, Root), ()> {
    let node = "enum_var";

    let start = convert;
    let start_range = convert.start_node(node)?;
    convert.update(start_range);

    let mut ty: Option<Root> = None;
    let mut data: Option<Box<Root>> = None;
    loop {
        if let Ok(range) = convert.end_node(node) {
            convert.update(range);
            break;
        } else if let Ok((range, val)) = convert.meta_string("ty") {
            convert.update(range);
            ty = Some(Root::Ty(val));
        } else if let Ok((range, val)) = parse_expr("ty", dirs, convert, ignored) {
            convert.update(range);
            ty = Some(val);
        } else if let Ok((range, val)) = convert.meta_string("data") {
            convert.update(range);
            data = Some(Box::new(Root::Str(val)));
        } else if let Ok((range, val)) = parse_expr("data", dirs, convert, ignored) {
            convert.update(range);
            data = Some(Box::new(val));
        } else {
            let range = convert.ignore();
            convert.update(range);
            ignored.push(range);
        }
    }

    let ty = ty.ok_or(())?;
    Ok((convert.subtract(start), Root::InstanceTy {ty: Box::new(ty), data: data}))
}

fn parse_ins(dirs: &[String], mut convert: Convert, ignored: &mut Vec<Range>) -> Result<(Range, Root), ()> {
    let node = "ins";

    let start = convert;
    let start_range = convert.start_node(node)?;
    convert.update(start_range);

    let mut class: Option<usize> = None;
    let mut ty: Option<Root> = None;
    let mut data: Option<Box<Root>> = None;
    loop {
        if let Ok(range) = convert.end_node(node) {
            convert.update(range);
            break;
        } else if let Ok((range, val)) = convert.meta_f64("class") {
            convert.update(range);
            class = Some(val as usize);
        } else if let Ok((range, val)) = parse_expr("ty", dirs, convert, ignored) {
            convert.update(range);
            ty = Some(val);
        } else if let Ok((range, val)) = parse_expr("data", dirs, convert, ignored) {
            convert.update(range);
            data = Some(Box::new(val));
        } else {
            let range = convert.ignore();
            convert.update(range);
            ignored.push(range);
        }
    }

    if let Some(ty) = ty {
        Ok((convert.subtract(start), Root::InstanceTy {ty: Box::new(ty), data: data}))
    } else {
        let class = class.ok_or(())?;
        Ok((convert.subtract(start), Root::Instance {class, data: data}))
    }
}

/// Parses an expression string.
pub fn parse_str(data: &str, dirs: &[String]) -> Result<Root, String> {
    use piston_meta::{parse_errstr, syntax_errstr};

    let syntax_src = include_str!("../assets/syntax.txt");
    let syntax = syntax_errstr(syntax_src)?;

    let mut meta_data = vec![];
    parse_errstr(&syntax, &data, &mut meta_data)?;

    // piston_meta::json::print(&meta_data);

    let convert = Convert::new(&meta_data);
    let mut ignored = vec![];
    match parse_expr("expr", dirs, convert, &mut ignored) {
        Err(()) => Err("Could not convert meta data".into()),
        Ok((_, expr)) => Ok(expr),
    }
}

/// Parses an expression source file.
pub fn parse(source: &str, dirs: &[String]) -> Result<Root, String> {
    use std::fs::File;
    use std::io::Read;

    let mut data_file = File::open(source).map_err(|err|
        format!("Could not open `{}`, {}", source, err))?;
    let mut data = String::new();
    data_file.read_to_string(&mut data).unwrap();

    parse_str(&data, dirs)
}
