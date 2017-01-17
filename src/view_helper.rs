use std::io::Write;
use std::str;

use handlebars::{Handlebars, RenderError, RenderContext, Helper, Context, JsonRender};

pub fn commify(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let param = h.param(0).unwrap();
    let s = param.value().as_string().unwrap();

    let mut result = String::with_capacity(s.len() + ((s.len() - 1) / 3));
    let first = s.len() % 3;
    result.push_str(&s[..first]);
    for chunk in s[first..].as_bytes().chunks(3) {
        if !result.is_empty() {
            result.push(',');
        }
        result.push_str(str::from_utf8(chunk).unwrap());
    }
    try!(rc.writer.write(result.into_bytes().as_ref()));
    Ok(())
}
