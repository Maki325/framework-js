use crate::{
  tpl_wrapper::TplWrapper,
  utils::{self, stringify::Stringify},
};
use lazy_static::lazy_static;
use num_traits::Zero;
use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};
use swc_ecma_ast::{
  ComputedPropName, Expr, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, Tpl,
};

pub fn style_object_to_string(obj: ObjectLit) -> Tpl {
  let mut tlp = TplWrapper::new();

  let mut is_first = true;
  for prop in obj.props {
    if !is_first {
      tlp.append_quasi(";");
    }
    is_first = false;

    // let (key, value) =
    match prop {
      PropOrSpread::Prop(prop) => match *prop {
        Prop::Shorthand(name) => {
          let str: &str = name.as_ref();
          tlp.append_quasi(process_style_name(str.to_string()));
          tlp.append_quasi(": ");
          tlp.append_expr(utils::call_framework_fn(
            "___FRAMEWORK_JS_STYLE_VALUE___",
            vec![Box::new(Expr::Ident(name)).into()],
          ));
        }
        Prop::KeyValue(KeyValueProp { key, value }) => {
          enum StrOrExpr<S: AsRef<str>> {
            Str(S),
            Expr(Expr),
          }

          impl<S: AsRef<str>> StrOrExpr<S> {
            fn as_expr(&self) -> Expr {
              match self {
                StrOrExpr::Str(s) => Expr::Lit(Lit::Str(s.as_ref().into())),
                StrOrExpr::Expr(e) => e.clone(),
              }
            }
          }

          let (key, _processed_key) = match key {
            PropName::Ident(i) => (
              StrOrExpr::Str(i.clone().stringify()),
              StrOrExpr::Str(process_style_name(i.stringify())),
            ),
            PropName::Str(str) => (
              StrOrExpr::Str(str.value.to_string().clone()),
              StrOrExpr::Str(process_style_name(str.value.to_string())),
            ),
            PropName::Computed(ComputedPropName { expr, .. }) => match *expr {
              Expr::Lit(lit) => match lit {
                Lit::Str(str) => (
                  StrOrExpr::Str(str.value.to_string()),
                  StrOrExpr::Str(process_style_name(str.value.to_string())),
                ),
                _ => unreachable!(),
              },
              Expr::Ident(name) => (
                StrOrExpr::Expr(Expr::Ident(name.clone())),
                StrOrExpr::Expr(utils::call_framework_fn(
                  "___FRAMEWORK_JS_STYLE_NAME___",
                  vec![Box::new(Expr::Ident(name)).into()],
                )),
              ),
              _ => unreachable!(),
            },
            _ => unreachable!(),
          };

          // let a = match processed_key {
          //   Err(expr) => &expr,
          //   Ok(a) => &Expr::Lit(Lit::Str(a.into())),
          // };

          let value = match &*value {
            Expr::Ident(_) | Expr::Tpl(_) => StrOrExpr::Expr(utils::call_framework_fn(
              "___FRAMEWORK_JS_STYLE_VALUE___",
              vec![value.into(), Box::new(key.as_expr()).into()],
            )),
            Expr::Lit(lit) => {
              match &key {
                StrOrExpr::Str(key) => match lit {
                  Lit::Str(s) => StrOrExpr::Str(s.value.to_string()),
                  Lit::JSXText(_) => unimplemented!(), // What is this exactly?
                  Lit::Num(num) => StrOrExpr::Str(if num.value == 0.0 {
                    "0".into()
                  } else {
                    if is_unitless_number(&key) {
                      num.to_string()
                    } else {
                      format!("{num}px")
                    }
                  }),
                  Lit::BigInt(num) => StrOrExpr::Str(if num.value.is_zero() {
                    "0".into()
                  } else {
                    if is_unitless_number(&key) {
                      num.value.to_string()
                    } else {
                      format!("{}px", num.value.to_string())
                    }
                  }),
                  Lit::Bool(b) => StrOrExpr::Str(b.value.to_string()),
                  Lit::Null(_) => unreachable!(),
                  Lit::Regex(_) => unreachable!(),
                },
                StrOrExpr::Expr(_) => StrOrExpr::Expr(utils::call_framework_fn(
                  "___FRAMEWORK_JS_STYLE_VALUE___",
                  vec![value.into(), Box::new(key.as_expr()).into()],
                )),
              }
            }
            // As unimplemented expressions pop up, I check if it can be implemented in rust
            // Or if I need to call it in JS
            _ => unimplemented!(),
          };

          match key {
            StrOrExpr::Str(s) => tlp.append_quasi(process_style_name(s)),
            StrOrExpr::Expr(expr) => tlp.append_expr(utils::call_framework_fn(
              "___FRAMEWORK_JS_STYLE_NAME___",
              vec![Box::new(expr).into()],
            )),
          }

          tlp.append_quasi(": ");

          match value {
            StrOrExpr::Str(s) => tlp.append_quasi(s),
            StrOrExpr::Expr(expr) => tlp.append_expr(expr),
          }

          // if let Err(expr) = processed_key {
          //   // A
          // } else {
          //   // B
          // }

          // unimplemented!();
          // let str: &str = key.as_ref();
          // tlp.append_quasi(process_style_name(str.to_string()));
          // tlp.append_quasi(": ");
          // tlp.append_expr(Expr::Ident(key));
        }
        _ => unreachable!(
          "Other `Prop` enumerations should never be present on the `style` component!"
        ),
      },
      PropOrSpread::Spread(spread) => {
        // B
      }
    }
  }

  return tlp.build();
}

lazy_static! {
  static ref STYLE_NAME_CACHE: Arc<Mutex<HashMap<String, &'static str>>> =
    Arc::new(Mutex::new(HashMap::new()));
}

fn process_style_name<'a>(name: String) -> &'static str {
  let map = &mut STYLE_NAME_CACHE.lock().unwrap();

  if let Some(name) = map.get(&name) {
    return (*name) as &'static str;
  }

  let value = escape_html(hyphenate_style_name(&name));
  map.insert(name.clone(), Box::new(value).leak());

  return map.get(&name).unwrap();
}

/**
 * Reimplemented from React
 * https://github.com/facebook/react/blob/9defcd56bc3cd53ac2901ed93f29218007010434/packages/react-dom-bindings/src/shared/hyphenateStyleName.js#L26
 *
 * Hyphenates a camelcased CSS property name, for example:
 *
 *   > hyphenateStyleName('backgroundColor')
 *   < "background-color"
 *   > hyphenateStyleName('MozTransition')
 *   < "-moz-transition"
 *   > hyphenateStyleName('msTransition')
 *   < "-ms-transition"
 *
 * As Modernizr suggests (http://modernizr.com/docs/#prefixed), an `ms` prefix
 * is converted to `-ms-`.
 */
fn hyphenate_style_name<S: AsRef<str>>(name: S) -> String {
  let name = name.as_ref().chars().collect::<Vec<_>>();
  let mut vec = Vec::<char>::with_capacity(name.len() + 10);

  if name.starts_with(&['m', 's']) && name[2].is_ascii_uppercase() {
    vec.push('-');
  }

  for c in name {
    if c.is_ascii_uppercase() {
      vec.push('-');
      vec.push(c.to_ascii_lowercase());
    } else {
      vec.push(c);
    }
  }

  return vec.into_iter().collect();
}

fn escape_html(value: String) -> String {
  let mut vec = Vec::<char>::with_capacity(value.len() + 20);

  for c in value.chars() {
    match c {
      '"' => vec.extend_from_slice(&['&', 'q', 'u', 'o', 't', ';']),
      '&' => vec.extend_from_slice(&['&', 'a', 'm', 'p', ';']),
      '\'' => vec.extend_from_slice(&['&', '#', 'x', '2', '7', ';']),
      '<' => vec.extend_from_slice(&['&', 'l', 't', ';']),
      '>' => vec.extend_from_slice(&['&', 'g', 't', ';']),
      c => vec.push(c),
    }
  }

  return vec.into_iter().collect();
}

const UNITLESS_NUMBERS: &'static [&'static str] = &[
  "animationIterationCount",
  "aspectRatio",
  "borderImageOutset",
  "borderImageSlice",
  "borderImageWidth",
  "boxFlex",
  "boxFlexGroup",
  "boxOrdinalGroup",
  "columnCount",
  "columns",
  "flex",
  "flexGrow",
  "flexPositive",
  "flexShrink",
  "flexNegative",
  "flexOrder",
  "gridArea",
  "gridRow",
  "gridRowEnd",
  "gridRowSpan",
  "gridRowStart",
  "gridColumn",
  "gridColumnEnd",
  "gridColumnSpan",
  "gridColumnStart",
  "fontWeight",
  "lineClamp",
  "lineHeight",
  "opacity",
  "order",
  "orphans",
  "scale",
  "tabSize",
  "widows",
  "zIndex",
  "zoom",
  "fillOpacity", // SVG-related properties
  "floodOpacity",
  "stopOpacity",
  "strokeDasharray",
  "strokeDashoffset",
  "strokeMiterlimit",
  "strokeOpacity",
  "strokeWidth",
  "MozAnimationIterationCount", // Known Prefixed Properties
  "MozBoxFlex",                 // TODO: Remove these since they shouldn't be used in modern code
  "MozBoxFlexGroup",
  "MozLineClamp",
  "msAnimationIterationCount",
  "msFlex",
  "msZoom",
  "msFlexGrow",
  "msFlexNegative",
  "msFlexOrder",
  "msFlexPositive",
  "msFlexShrink",
  "msGridColumn",
  "msGridColumnSpan",
  "msGridRow",
  "msGridRowSpan",
  "WebkitAnimationIterationCount",
  "WebkitBoxFlex",
  "WebKitBoxFlexGroup",
  "WebkitBoxOrdinalGroup",
  "WebkitColumnCount",
  "WebkitColumns",
  "WebkitFlex",
  "WebkitFlexGrow",
  "WebkitFlexPositive",
  "WebkitFlexShrink",
  "WebkitLineClamp",
];

fn is_unitless_number(name: &str) -> bool {
  return UNITLESS_NUMBERS.iter().any(|a| a.eq(&name));
}
