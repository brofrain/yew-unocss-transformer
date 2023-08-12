## ⚠️ This crate is no longer maintained ⚠️

The Variant Groups feature of this package is no longer in sync with the present versions of UnoCSS, therefore you should avoid using it. Also, utilizing a dedicated UnoCSS preset to parse Rust files is no longer necessary.\
[unocss-classes](https://crates.io/crates/unocss-classes) has been created as the replacement. It adds a few new features and aims to support more Rust frontend frameworks than just Yew.

# yew-unocss-transformer

> Yew classes macro with variant group transformer for UnoCSS

This crate provides `uno!` macro that applies [@unocss/transformer-variant-group](https://github.com/unocss/unocss/tree/main/packages/transformer-variant-group) to given string literals.

The transformation is executed Rust-side and allows HTML elements with valid classes to be generated. **`.rs` files are not however parsed correctly by UnoCSS by default**. Use this macro along with [unocss-preset-yew](https://www.npmjs.com/package/unocss-preset-yew) so CSS classes can be generated from Rust codebase.

The macro unlike `classes!` does not enforce using single class per string (`uno!("text-blue fw800")` works just fine). Only string literals are allowed however - anything else cannot be transformed anyway. For dynamic classes use standard `classes!` macro along with populating UnoCSS [safelist](https://github.com/unocss/unocss#safelist).

## Example

```rust
use yew::prelude::*;
use yew_unocss_transformer::uno;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div id="app">
            <input
                class={
                    uno!(
                        "w72",
                        "p-(x5 y.5)", // px5 py.5
                        "text-(center red)", // text-center text-red
                        "fw700",
                        "bg-green",
                        "rounded",
                        "border-(1 blue/30)", // border-1 border-blue/30
                        "placeholder:(italic text-sm text-secondary/75)", // placeholder:italic placeholder:text-sm placeholder:text-secondary/75
                        "outline-(~ 2 offset-0 transparent)", // outline outline-2 outline-offset-0 outline-transparent
                        "hover:outline-yellow !focus:outline-orange",
                        "transition-all",
                    )
                }
            />
        </div>
    }
}
```

The code above is equivalent to:

```rust
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div id="app">
            <input
                class={
                    classes!(
                        "w72",
                        "px5",
                        "py.5",
                        "text-center",
                        "text-red",
                        "fw700",
                        "bg-green",
                        "rounded",
                        "border-1",
                        "border-blue/30",
                        "placeholder:italic",
                        "placeholder:text-sm",
                        "placeholder:text-secondary/75",
                        "outline",
                        "outline-2",
                        "outline-offset-0",
                        "outline-transparent",
                        "hover:outline-yellow",
                        "!focus:outline-orange",
                        "transition-all",
                    )
                }
            />
        </div>
    }
}
```

But it's so much shorter!

<details>
<summary><a href="https://www.npmjs.com/package/unocss-preset-yew">unocss-preset-yew</a> would generate this CSS code:</summary>

```css
.w72 {
  width: 18rem;
}
.border-1 {
  border-width: 1px;
  border-style: solid;
}
.border-blue\/30 {
  border-color: rgba(96, 165, 250, 0.3);
}
.rounded {
  border-radius: 0.25rem;
}
.bg-green {
  --un-bg-opacity: 1;
  background-color: rgba(74, 222, 128, var(--un-bg-opacity));
}
.p-x5 {
  padding-left: 1.25rem;
  padding-right: 1.25rem;
}
.p-y\.5 {
  padding-top: 0.125rem;
  padding-bottom: 0.125rem;
}
.text-center {
  text-align: center;
}
.placeholder\:text-sm::placeholder {
  font-size: 0.875rem;
  line-height: 1.25rem;
}
.fw700 {
  font-weight: 700;
}
.placeholder\:italic::placeholder {
  font-style: italic;
}
.text-red {
  --un-text-opacity: 1;
  color: rgba(248, 113, 113, var(--un-text-opacity));
}
.outline-2 {
  outline-width: 2px;
}
.\!focus\:outline-orange:focus {
  --un-outline-color-opacity: 1 !important;
  outline-color: rgba(251, 146, 60, var(--un-outline-color-opacity)) !important;
}
.hover\:outline-yellow:hover {
  --un-outline-color-opacity: 1;
  outline-color: rgba(250, 204, 21, var(--un-outline-color-opacity));
}
.outline-transparent {
  outline-color: transparent;
}
.outline-offset-0 {
  outline-offset: 0px;
}
.outline {
  outline-style: solid;
}
.transition-all {
  transition-property: all;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 150ms;
}
```

</details>

## Using `uno!` macro globally

```rust
#[macro_use]
extern crate yew_unocss_transformer;
```

## Using Yew with UnoCSS

[@unocss/cli](https://github.com/unocss/unocss/tree/main/packages/cli) can be used to generate `uno.css` file that can be then imported in `index.html` used by Trunk.\
It is also possible to run Vite with UnoCSS in parallel alongside Trunk.

## License

[MIT License](https://opensource.org/licenses/MIT)

Copyright (c) 2022-PRESENT Kajetan Welc

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
