// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*
 * The compiler code necessary to support the env! extension.  Eventually this
 * should all get sucked into either the compiler syntax extension plugin
 * interface.
 */

use ast;
use codemap::span;
use ext::base::*;
use ext::base;
use ext::build::AstBuilder;

use std::os;

pub fn expand_option_env(ext_cx: @ExtCtxt, sp: span, tts: &[ast::token_tree])
    -> base::MacResult {
    let var = get_single_str_from_tts(ext_cx, sp, tts, "option_env!");

    let e = match os::getenv(var) {
      None => quote_expr!(::std::option::None),
      Some(s) => quote_expr!(::std::option::Some($s))
    };
    MRExpr(e)
}

pub fn expand_env(ext_cx: @ExtCtxt, sp: span, tts: &[ast::token_tree])
    -> base::MacResult {
    let exprs = get_exprs_from_tts(ext_cx, sp, tts);

    if exprs.len() == 0 {
        ext_cx.span_fatal(sp, "env! takes 1 or 2 arguments");
    }

    let var = expr_to_str(ext_cx, exprs[0], "expected string literal");
    let msg = match exprs.len() {
        1 => fmt!("Environment variable %s not defined", var).to_managed(),
        2 => expr_to_str(ext_cx, exprs[1], "expected string literal"),
        _ => ext_cx.span_fatal(sp, "env! takes 1 or 2 arguments")
    };

    let e = match os::getenv(var) {
        None => ext_cx.span_fatal(sp, msg),
        Some(s) => ext_cx.expr_str(sp, s.to_managed())
    };
    MRExpr(e)
}
