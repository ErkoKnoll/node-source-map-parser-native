use neon::prelude::*;
use sourcemap::{Error, SourceMap};
use std::{cell::RefCell, collections::HashMap};

enum LookupStatus {
    SourceMapNotFound,
    LookupFailed,
    LookupSuccess(LookupResult),
}

struct LookupResult {
    line: u32,
    column: u32,
    source: String,
}

thread_local! {
    static COUNTER: RefCell<u32> = RefCell::new(0);
    static PARSED_MAPS: RefCell<HashMap<u32, SourceMap>> = RefCell::new(HashMap::new());
}

fn parse_source_map(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let source_map = cx.argument::<JsString>(0)?.value();
    let source_map = source_map.as_bytes();

    let index = COUNTER.with(|counter| {
        *counter.borrow_mut() += 1;
        *counter.borrow()
    });

    let parse_error: Option<Error> = PARSED_MAPS.with(|parsed_maps| {
        let parsed_map = SourceMap::from_reader(source_map);
        match parsed_map {
            Ok(parsed_map) => {
                parsed_maps.borrow_mut().insert(index, parsed_map);
                None
            }
            Err(err) => Some(err)
        }
    });

    match parse_error {
        Some(e) => cx.throw_error(String::from(
            format!("Failed to parse source map - Reason: {}", e.to_string()),
        )),
        None => Ok(cx.number(index as f64)),
    }
}

fn lookup_original_position(mut cx: FunctionContext) -> JsResult<JsObject> {
    let index = cx.argument::<JsNumber>(0)?.value() as u32;
    let line = cx.argument::<JsNumber>(1)?.value() as u32;
    let column = cx.argument::<JsNumber>(2)?.value() as u32;

    let lookup_status: LookupStatus = PARSED_MAPS.with(|parsed_maps| {
        let parsed_map = parsed_maps.borrow();
        let parsed_map = parsed_map.get(&index);

        if let Some(parsed_map) = parsed_map {
            let token = parsed_map.lookup_token(line, column);

            if let Some(token) = token {
                LookupStatus::LookupSuccess(LookupResult {
                    line: token.get_src_line(),
                    column: token.get_src_col(),
                    source: match token.get_source() {
                        Some(v) => v.to_string(),
                        None => String::from(""),
                    },
                })
            } else {
                LookupStatus::LookupFailed
            }
        } else {
            LookupStatus::SourceMapNotFound
        }
    });

    match lookup_status {
        LookupStatus::LookupSuccess(lookup_result) => {
            let js_object = JsObject::new(&mut cx);
            let js_line = cx.number(lookup_result.line);
            let js_column = cx.number(lookup_result.column);
            let js_source = cx.string(lookup_result.source);
            js_object.set(&mut cx, "line", js_line).unwrap();
            js_object.set(&mut cx, "column", js_column).unwrap();
            js_object.set(&mut cx, "source", js_source).unwrap();

            Ok(js_object)
        }
        LookupStatus::SourceMapNotFound => cx.throw_error(String::from(
            "Source map was not found, did you dispose it?",
        )),
        LookupStatus::LookupFailed => cx.throw_error(String::from(
            "Failed to lookup original position for given line and column",
        )),
    }
}

fn dispose(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let index = cx.argument::<JsNumber>(0)?.value() as u32;

    let maps_left = PARSED_MAPS.with(|parsed_maps| {
        parsed_maps.borrow_mut().remove(&index);
        parsed_maps.borrow().len()
    });

    Ok(cx.number(maps_left as f64))
}

register_module!(mut cx, {
    cx.export_function("parseSourceMap", parse_source_map)?;
    cx.export_function("lookupOriginalPosition", lookup_original_position)?;
    cx.export_function("dispose", dispose)?;

    Ok(())
});
