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
                        path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z" {}
                    }
                }
                p .empty-text { "There is no list available" }
                h2 .empty-title { "Create grocery list" }
                form hx-post="/create" hx-target="body" style="max-width: 280px; width: 100%;" {
                    button .btn.btn-primary type="submit" style="display: flex; align-items: center; justify-content: center; gap: 8px;" {
                        svg style="width: 20px; height: 20px;" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" {}
                        }
                        span { "Create grocery list" }
                    }
                }
            }
        }
    }
}

pub fn lists_view(lists: &HashMap<String, GroceryList>) -> Markup {
    html! {
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
            h1 { "Name your list" }
            form hx-post="/create" hx-target="body" {
                input type="text" name="name" placeholder="Enter grocery list name" required autofocus;
                button .btn.btn-primary type="submit" { "Create grocery list" }
            }
        }
    }
}
