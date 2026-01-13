use crate::models::{AddItemForm, Item};
use crate::state::{AppContext, save_data};
use crate::templates::{layout, list as list_tpl};
use axum::{
    Form,
    extract::{Path, State},
    http::HeaderMap,
    response::{Html, IntoResponse},
};

fn broadcast_update(ctx: &AppContext, headers: &HeaderMap) {
    let client_id = headers
        .get("X-Client-Id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let event = serde_json::json!({
        "type": "reload",
        "client_id": client_id
    });
    let _ = ctx.update_tx.send(event.to_string());
}

pub async fn view_list(Path(id): Path<String>, State(ctx): State<AppContext>) -> impl IntoResponse {
    let lists = ctx.state.read().unwrap();
    let Some(list) = lists.get(&id) else {
        return Html(layout::render(maud::html! { "List not found" }).into_string());
    };

    let content = list_tpl::render(&id, list);
    Html(layout::render(content).into_string())
}

pub async fn add_item(
    Path(id): Path<String>,
    State(ctx): State<AppContext>,
    headers: HeaderMap,
    Form(form): Form<AddItemForm>,
) -> impl IntoResponse {
    if let Some(list) = ctx.state.write().unwrap().get_mut(&id) {
        list.items.push(Item {
            name: form.item,
            completed: false,
        });
    }
    save_data(&ctx.state).await;
    broadcast_update(&ctx, &headers);
    view_list(Path(id), State(ctx)).await
}

pub async fn edit_item(
    Path((id, idx)): Path<(String, usize)>,
    State(ctx): State<AppContext>,
    headers: HeaderMap,
    Form(form): Form<AddItemForm>,
) -> impl IntoResponse {
    if let Some(list) = ctx.state.write().unwrap().get_mut(&id) {
        if let Some(item) = list.items.get_mut(idx) {
            item.name = form.item;
        }
    }
    save_data(&ctx.state).await;
    broadcast_update(&ctx, &headers);
    view_list(Path(id), State(ctx)).await
}

pub async fn delete_item(
    Path((id, idx)): Path<(String, usize)>,
    State(ctx): State<AppContext>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Some(list) = ctx.state.write().unwrap().get_mut(&id) {
        if idx < list.items.len() {
            list.items.remove(idx);
        }
    }
    save_data(&ctx.state).await;
    broadcast_update(&ctx, &headers);
    view_list(Path(id), State(ctx)).await
}

pub async fn toggle_item(
    Path((id, idx)): Path<(String, usize)>,
    State(ctx): State<AppContext>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Some(list) = ctx.state.write().unwrap().get_mut(&id) {
        if let Some(item) = list.items.get_mut(idx) {
            item.completed = !item.completed;
        }
    }
    save_data(&ctx.state).await;
    broadcast_update(&ctx, &headers);
    view_list(Path(id), State(ctx)).await
}

pub async fn toggle_show_completed(
    Path(id): Path<String>,
    State(ctx): State<AppContext>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Some(list) = ctx.state.write().unwrap().get_mut(&id) {
        list.show_completed = !list.show_completed;
    }
    save_data(&ctx.state).await;
    broadcast_update(&ctx, &headers);
    view_list(Path(id), State(ctx)).await
}

pub async fn delete_completed_items(
    Path(id): Path<String>,
    State(ctx): State<AppContext>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Some(list) = ctx.state.write().unwrap().get_mut(&id) {
        list.items.retain(|item| !item.completed);
    }
    save_data(&ctx.state).await;
    broadcast_update(&ctx, &headers);
    view_list(Path(id), State(ctx)).await
}

pub async fn sort_list(
    Path(id): Path<String>,
    State(ctx): State<AppContext>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Some(list) = ctx.state.write().unwrap().get_mut(&id) {
        list.items
            .sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    }
    save_data(&ctx.state).await;
    broadcast_update(&ctx, &headers);
    view_list(Path(id), State(ctx)).await
}

pub async fn delete_list(
    Path(id): Path<String>,
    State(ctx): State<AppContext>,
    headers: HeaderMap,
) -> impl IntoResponse {
    ctx.state.write().unwrap().remove(&id);
    save_data(&ctx.state).await;
    broadcast_update(&ctx, &headers);

    let lists = ctx.state.read().unwrap();
    let content = crate::templates::home::lists_view(&lists);
    Html(layout::render(content).into_string())
}
