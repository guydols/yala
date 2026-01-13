use crate::models::GroceryList;
use maud::{Markup, PreEscaped, html};

pub fn render(id: &str, list: &GroceryList) -> Markup {
    let visible_items: Vec<_> = list
        .items
        .iter()
        .enumerate()
        .filter(|(_, item)| list.show_completed || !item.completed)
        .collect();

    let hide_completed_attr = if list.show_completed { "false" } else { "true" };

    html! {
        div .container data-hide-completed=(hide_completed_attr) {
            div .header {
                a .back-btn href="/" { "←" }
                h1 { (list.name) }
                button .menu-btn onclick="document.getElementById('menu').style.display='block'" { "⋮" }
            }

            @for (idx, item) in visible_items {
                @let item_class = if item.completed { "item completed" } else { "item" };
                @let checkbox_class = if item.completed { "checkbox checked" } else { "checkbox" };
                @let delete_url = format!("/list/{}/delete-item/{}", id, idx);
                @let edit_call = format!("window.editItem(this, '{}', {})", id, idx);
                @let checkbox_click = format!("window.handleCheckboxClick(event, '{}', {})", id, idx);
                div class=(item_class) data-delete-url=(delete_url) {
                    div class=(checkbox_class)
                        onclick=(PreEscaped(&checkbox_click)) {}
                    span .item-text onclick=(PreEscaped(&edit_call)) { (item.name) }
                }
            }

            div .add-item {
                form hx-post={"/list/" (id) "/add"} hx-target="body" hx-swap="outerHTML" {
                    div .checkbox {}
                    input #add-input type="text" name="item" placeholder="Add item" required;
                }
            }

            (menu(id, list.show_completed))
            (confirm_modal(id))
        }
    }
}

fn menu(id: &str, show_completed: bool) -> Markup {
    html! {
        div #menu .menu style="display:none;" {
            div .menu-item hx-post={"/list/" (id) "/sort"} hx-target="body" {
                svg fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4h13M3 8h9m-9 4h6m4 0l4-4m0 0l4 4m-4-4v12" {}
                }
                span { "Sort A-Z" }
            }
            div .menu-item onclick=(PreEscaped(&format!("window.handleToggleCompleted('{}')", id))) {
                svg fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" {}
                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" {}
                }
                @if show_completed {
                    span { "Hide completed" }
                } @else {
                    span { "Show completed" }
                }
            }
            div .menu-item onclick=(PreEscaped(&format!("window.handleDeleteCompleted('{}')", id))) {
                svg fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" {}
                }
                span { "Delete completed items" }
            }
            div .menu-item.danger onclick="document.getElementById('confirm').style.display='flex';document.getElementById('menu').style.display='none';" {
                svg fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" {}
                }
                span { "Delete list" }
            }
        }
    }
}

fn confirm_modal(id: &str) -> Markup {
    html! {
        div #confirm .modal style="display:none;" {
            div .modal-content {
                div .modal-title { "Are you sure?" }
                button .btn.btn-danger hx-post={"/list/" (id) "/delete"} hx-target="body" style="display: flex; align-items: center; justify-content: center; gap: 8px;" {
                    svg style="width: 20px; height: 20px;" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                        path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" {}
                    }
                    span { "Yes, delete" }
                }
            }
        }
    }
}
