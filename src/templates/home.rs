use crate::models::GroceryList;
use maud::{Markup, html};
use std::collections::HashMap;

pub fn render(lists: &HashMap<String, GroceryList>) -> Markup {
    if lists.is_empty() {
        empty_state()
    } else {
        lists_view(lists)
    }
}

fn empty_state() -> Markup {
    html! {
        div .container {
            div .empty-state {
                div .empty-icon {
                    svg fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                        path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01" {}
                    }
                }
                p .empty-text { "There is no list available" }
                h2 .empty-title { "Create list" }
                a href="/new" style="max-width: 280px; width: 100%;" {
                    button .btn.btn-primary type="submit" style="display: flex; align-items: center; justify-content: center; gap: 8px;" {
                        svg style="width: 20px; height: 20px;" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" {}
                        }
                        span { "Create list" }
                    }
                }
            }
        }
    }
}

pub fn lists_view(lists: &HashMap<String, GroceryList>) -> Markup {
    html! {
        h1 class="toptitle" { "Lists" }
        div .container {
            @for (id, list) in lists.iter() {
                a .list-item href={"/list/" (id)} {
                    span .list-name { (list.name) }
                    div style="display: flex; align-items: center; gap: 12px;" {
                        span .item-count { (list.items.len()) }
                        span .arrow { "›" }
                    }
                }
            }
            a .fab href="/new" { "+" }
        }
    }
}

pub fn new_list_form() -> Markup {
    html! {
        div .container {
            h1 class="toptitle" { "Name your list" }
            form hx-post="/create" hx-target="body" {
                input type="text" name="name" placeholder="Enter list name" required autofocus;
                button .btn.btn-primary type="submit" { "Create list" }
            }
        }
    }
}
