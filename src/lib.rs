use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use regex::{Captures, Regex};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Expr, ExprLit, Lit, LitStr, Token};

static CLASS_GROUP_REG: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"((?:[!\w+:_/-]|\[&?>?:?.*\])+?)([:-])\(((?:[~!\w\s:/\\,%#.$-]|\[.*?\])*?)\)")
        .unwrap()
});
static WHITESPACE_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s").unwrap());
static IMPORTANCE_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(!?)(.*)").unwrap());

const SEPARATORS: [&str; 2] = ["-", ":"];

fn shallow_transform(str: &str) -> String {
    CLASS_GROUP_REG
        .replace_all(str, |caps: &Captures| {
            if !SEPARATORS.contains(&&caps[2]) {
                return caps[0].to_string();
            }

            WHITESPACE_REG
                .split(&caps[3])
                .filter(|item| !item.is_empty())
                .map(|item| {
                    if item == "~" {
                        caps[1].to_string()
                    } else {
                        IMPORTANCE_REG
                            .replace(item, format!("${{1}}{}{}${{2}}", &caps[1], &caps[2]))
                            .to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join(" ")
        })
        .into_owned()
}

fn transform(str: &str) -> String {
    let mut depth = 10_u8;
    let mut previous = String::from(str);

    loop {
        let transformed = shallow_transform(&previous);
        depth -= 1;

        if transformed == previous || depth == 0 {
            break previous;
        }

        previous = transformed
    }
}

#[derive(Clone, Debug)]
struct UnoClassExpr(LitStr);

const ERROR_MSG: &str = "Only string literals are allowed (hint: use classes! macro)";

impl Parse for UnoClassExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.parse()? {
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit_str),
                ..
            }) => {
                let transformed_value = transform(&lit_str.value());
                let new_lit_str = LitStr::new(&transformed_value, lit_str.span());

                Ok(Self(new_lit_str))
            }
            expr => Err(syn::Error::new(expr.span(), ERROR_MSG)),
        }
    }
}

#[derive(Debug, Clone)]
struct UnoClasses(Punctuated<UnoClassExpr, Token![,]>);

impl Parse for UnoClasses {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse_terminated(UnoClassExpr::parse).map(Self)
    }
}

impl ToTokens for UnoClasses {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let n = self.0.len();

        let push_classes = self.0.iter().map(|class_expr| {
            let UnoClassExpr(class) = class_expr;
            quote! {
                __yew_classes.push(#class);
            }
        });

        tokens.extend(quote! {
            {
                let mut __yew_classes = ::yew::html::Classes::with_capacity(#n);
                #(#push_classes)*
                __yew_classes
            }
        });
    }
}

/// A situational substitute of the [yew::classes!](https://docs.rs/yew/latest/yew/macro.classes.html) macro that additionally
/// applies [Variant Group Transformation](https://github.com/unocss/unocss/tree/main/packages/transformer-variant-group)
/// for usage with [UnoCSS](https://github.com/unocss/unocss).
///
/// The macro, same as [yew::classes!](https://docs.rs/yew/latest/yew/macro.classes.html), takes a list of items
/// and returns a [Classes](https://docs.rs/yew/latest/yew/html/struct.Classes.html) instance.
/// Unlike [yew::classes!](https://docs.rs/yew/latest/yew/macro.classes.html), [uno!](#) does not enforce using a single class
/// per string (e.g. `uno!("text-blue fw800")` works just fine).
/// The items, however, must be all string literals - other types cannot be transformed anyway.
///
/// You should use the macro only for [UnoCSS](https://github.com/unocss/unocss) utils. For dynamic classes you should stick
/// with the classic [yew::classes!](https://docs.rs/yew/latest/yew/macro.classes.html) macro and expand
/// [UnoCSS safelist](https://github.com/unocss/unocss#safelist), if necessary.
///
/// The transformation is executed Rust-side and allows HTML elements with valid classes to be generated. **`.rs` files are not
/// however parsed correctly by [UnoCSS](https://github.com/unocss/unocss) by default**. Use this macro along with
/// [unocss-preset-yew](https://www.npmjs.com/package/unocss-preset-yew) so CSS classes can be generated from Rust codebase.
///
/// # Example
///
/// ```
/// use yew_unocss_transformer::uno;
///
/// assert_eq!(uno!("text-red"), yew::classes!("text-red"));
///
/// assert_eq!(uno!("text-(red sm)"), yew::classes!("text-red", "text-sm"));
///
/// assert_eq!(
///     uno!("text-(blue lg)", "placeholder:(italic text-(red sm))"),
///     yew::classes!(
///         "text-blue",
///         "text-lg",
///         "placeholder:italic",
///         "placeholder:text-red",
///         "placeholder:text-sm"
///     )
/// );
///
/// let dynamic_classes_from_vector = vec!["my-simple-button", "my-simple-button--disabled"];
/// assert_eq!(
///     yew::classes!(dynamic_classes_from_vector.clone(), uno!("text-(red sm)")),
///     yew::classes!(dynamic_classes_from_vector.clone(), "text-red", "text-sm")
/// );
/// ```
#[proc_macro]
pub fn uno(input: TokenStream) -> TokenStream {
    let classes = parse_macro_input!(input as UnoClasses);
    TokenStream::from(classes.into_token_stream())
}
