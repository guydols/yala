use crate::models::{CreateForm, GroceryList};
use crate::state::{AppContext, save_data};
use crate::templates::{home as home_tpl, layout};
use axum::{
    Form,
    extract::State,
    response::{Html, IntoResponse},
};

pub async fn home(State(ctx): State<AppContext>) -> Html<String> {
    let lists = ctx.state.read().unwrap();
    let content = home_tpl::render(&lists);
    Html(layout::render(content).into_string())
}

pub async fn new_list_form() -> Html<String> {
    let content = home_tpl::new_list_form();
    Html(layout::render(content).into_string())
}

async fn lists_view(State(ctx): State<AppContext>) -> Html<String> {
    let lists = ctx.state.read().unwrap();
    let content = home_tpl::lists_view(&lists);
    Html(layout::render(content).into_string())
}

pub async fn create_list(
    State(ctx): State<AppContext>,
    Form(form): Form<CreateForm>,
) -> impl IntoResponse {
    let id = form.name.to_lowercase().replace(" ", "-");
    let list = GroceryList {
        name: form.name,
        items: vec![],
        show_completed: true,
    };

    ctx.state.write().unwrap().insert(id.clone(), list);
    save_data(&ctx.state).await;

    lists_view(State(ctx)).await
}
