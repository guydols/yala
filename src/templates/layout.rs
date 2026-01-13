use crate::assets::{scripts, styles};
use maud::{DOCTYPE, Markup, PreEscaped, html};

pub fn render(content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1, user-scalable=no";
                title { "Grocery Lists" }
                script src="https://unpkg.com/htmx.org@1.9.10" {}
                script src="https://cdnjs.cloudflare.com/ajax/libs/hammer.js/2.0.8/hammer.min.js" {}
                script src="https://cdnjs.cloudflare.com/ajax/libs/animejs/3.2.1/anime.min.js" {}
                style { (PreEscaped(styles::CSS)) }
            }
            body {
                (content)
            }
            script { (PreEscaped(scripts::JS)) }
        }
    }
}
