use std::io::Write;
use std::str;

use handlebars::{Handlebars, RenderError, RenderContext, Helper, Context, JsonRender};

pub fn commify(c: &Context, h: &Helper, _: &Handlebars, rc: &mut RenderContext)
               -> Result<(), RenderError> {
    let param = h.params().get(0).unwrap();

    // get value from context data
    // rc.get_path() is current json parent path, you should always use it like this
    // param is the key of value you want to display
    let s = c.navigate(rc.get_path(), param).render();

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
