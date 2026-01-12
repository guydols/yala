use axum::{
    Form, Router,
    extract::{Path, State},
    response::{Html, IntoResponse, Sse, sse::Event},
    routing::{get, post},
};
use futures::stream::{Stream, StreamExt};
use maud::{DOCTYPE, Markup, PreEscaped, html};
use notify::{RecursiveMode, Result as NotifyResult, Watcher};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::Infallible,
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio::fs;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

const STORE: &'static str = "lists.json";
const BIND: &'static str = "0.0.0.0:3000";

#[derive(Clone, Serialize, Deserialize)]
struct Item {
    name: String,
    completed: bool,
}

#[derive(Clone, Serialize, Deserialize)]
struct GroceryList {
    name: String,
    items: Vec<Item>,
    show_completed: bool,
}

type AppState = Arc<RwLock<HashMap<String, GroceryList>>>;

#[derive(Clone)]
struct AppContext {
    state: AppState,
    update_tx: broadcast::Sender<()>,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(RwLock::new(load_data().await));
    let (update_tx, _) = broadcast::channel(100);

    let ctx = AppContext {
        state: state.clone(),
        update_tx: update_tx.clone(),
    };

    // Spawn file watcher
    tokio::spawn(watch_file(update_tx.clone()));

    let app = Router::new()
        .route("/", get(home))
        .route("/new", get(new_list_form))
        .route("/create", post(create_list))
        .route("/list/:id", get(view_list))
        .route("/list/:id/add", post(add_item))
        .route("/list/:id/toggle/:idx", post(toggle_item))
        .route("/list/:id/edit/:idx", post(edit_item))
        .route("/list/:id/delete-item/:idx", post(delete_item))
        .route("/list/:id/toggle-completed", post(toggle_show_completed))
        .route("/list/:id/delete-completed", post(delete_completed_items))
        .route("/list/:id/sort", post(sort_list))
        .route("/list/:id/delete", post(delete_list))
        .route("/events", get(sse_handler))
        .with_state(ctx);

    let listener = tokio::net::TcpListener::bind(BIND).await.unwrap();
    println!("Server running on http://{}", BIND);
    axum::serve(listener, app).await.unwrap();
}

async fn watch_file(tx: broadcast::Sender<()>) {
    let (notify_tx, mut notify_rx) = tokio::sync::mpsc::channel(100);

    let mut watcher = notify::recommended_watcher(move |res: NotifyResult<notify::Event>| {
        if let Ok(event) = res {
            if event.kind.is_modify() {
                let _ = notify_tx.blocking_send(());
            }
        }
    })
    .unwrap();

    let _ = watcher.watch(std::path::Path::new(STORE), RecursiveMode::NonRecursive);

    while let Some(_) = notify_rx.recv().await {
        tokio::time::sleep(Duration::from_millis(100)).await; // Debounce
        let _ = tx.send(());
    }
}

async fn sse_handler(
    State(ctx): State<AppContext>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = ctx.update_tx.subscribe();
    let stream = BroadcastStream::new(rx).map(|_| Ok(Event::default().data("reload")));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(30))
            .text("keep-alive"),
    )
}

async fn load_data() -> HashMap<String, GroceryList> {
    fs::read_to_string(STORE)
        .await
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

async fn save_data(state: &AppState) {
    let data = state.read().unwrap().clone();
    if let Ok(json) = serde_json::to_string_pretty(&data) {
        let _ = fs::write(STORE, json).await;
    }
}

fn layout(content: Markup) -> Markup {
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

                style {
                    r#"
                    * {
                        margin: 0;
                        padding: 0;
                        box-sizing: border-box;
                    }

                    body {
                        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
                        background: #111827;
                        color: #f3f4f6;
                        min-height: 100vh;
                        overscroll-behavior: none;
                    }

                    .container {
                        max-width: 672px;
                        margin: 0 auto;
                        padding: 24px;
                    }

                    h1 {
                        font-size: 24px;
                        font-weight: 500;
                        margin-bottom: 24px;
                        text-align: center;
                        color: #f3f4f6;
                    }

                    input[type="text"] {
                        width: 100%;
                        padding: 16px;
                        background: #1f2937;
                        border: 1px solid #374151;
                        border-radius: 8px;
                        color: #f3f4f6;
                        font-size: 16px;
                        margin-bottom: 16px;
                        transition: border-color 0.2s;
                    }

                    input[type="text"]:focus {
                        outline: none;
                        border-color: #6b7280;
                    }

                    input::placeholder {
                        color: #6b7280;
                    }

                    button, .btn {
                        width: 100%;
                        padding: 16px;
                        border: none;
                        border-radius: 8px;
                        font-size: 16px;
                        font-weight: 500;
                        cursor: pointer;
                        transition: all 0.2s;
                    }

                    button:hover, .btn:hover {
                        opacity: 0.9;
                    }

                    .btn-primary {
                        background: #2563eb;
                        color: white;
                    }

                    .btn-primary:hover {
                        background: #1d4ed8;
                    }

                    .btn-danger {
                        background: #dc2626;
                        color: white;
                    }

                    .btn-danger:hover {
                        background: #b91c1c;
                    }

                    .list-item {
                        display: flex;
                        justify-content: space-between;
                        align-items: center;
                        padding: 20px;
                        border-radius: 8px;
                        background: #1f2937;
                        border: 1px solid #374151;
                        margin-bottom: 12px;
                        cursor: pointer;
                        text-decoration: none;
                        color: inherit;
                        transition: background 0.2s;
                    }

                    .list-item:hover {
                        background: #293548;
                    }

                    .list-name {
                        font-size: 18px;
                        font-weight: 500;
                    }

                    .item-count {
                        color: #9ca3af;
                        margin-right: 12px;
                    }

                    .arrow {
                        color: #9ca3af;
                        font-size: 20px;
                    }

                    .fab {
                        position: fixed;
                        bottom: 30px;
                        right: 30px;
                        width: 56px;
                        height: 56px;
                        background: #2563eb;
                        border-radius: 50%;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        font-size: 28px;
                        color: white;
                        box-shadow: 0 4px 12px rgba(37, 99, 235, 0.4);
                        text-decoration: none;
                        font-weight: 300;
                        line-height: 1;
                    }

                    .fab:hover {
                        background: #1d4ed8;
                    }

                    .header {
                        display: flex;
                        align-items: center;
                        justify-content: space-between;
                        margin-bottom: 24px;
                        padding: 16px 24px;
                        background: #1f2937;
                        border-bottom: 1px solid #374151;
                        position: sticky;
                        top: 0;
                        z-index: 10;
                    }

                    .back-btn, .menu-btn {
                        width: 40px;
                        height: 40px;
                        background: transparent;
                        border-radius: 50%;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        cursor: pointer;
                        border: none;
                        color: #f3f4f6;
                        text-decoration: none;
                        font-size: 20px;
                        transition: background 0.2s;
                    }

                    .back-btn:hover, .menu-btn:hover {
                        background: #374151;
                    }

                    .item {
                        display: flex;
                        align-items: center;
                        padding: 16px 24px;
                        border-bottom: 1px solid #1f2937;
                        position: relative;
                        cursor: grab;
                        user-select: none;
                        -webkit-user-select: none;
                        touch-action: pan-y;
                        transition: background 0.2s;
                    }

                    .item:hover {
                        background: #1a1f2e;
                    }

                    .item:active {
                        cursor: grabbing;
                    }

                    .checkbox {
                        width: 24px;
                        height: 24px;
                        border: 2px solid #4b5563;
                        border-radius: 50%;
                        margin-right: 12px;
                        cursor: pointer;
                        flex-shrink: 0;
                        transition: all 0.2s;
                    }

                    .checkbox.checked {
                        background: #2563eb;
                        border-color: #2563eb;
                        position: relative;
                    }

                    .checkbox.checked::after {
                        content: '✓';
                        position: absolute;
                        color: white;
                        font-size: 14px;
                        top: 50%;
                        left: 50%;
                        transform: translate(-50%, -50%);
                    }

                    .item.completed .item-text {
                        text-decoration: line-through;
                        color: #6b7280;
                    }

                    .item-text {
                        flex: 1;
                        cursor: text;
                        padding: 4px;
                        color: #f3f4f6;
                    }

                    .add-item {
                        display: flex;
                        align-items: center;
                        padding: 16px 24px;
                        margin-top: 8px;
                    }

                    .add-item form {
                        width: 100%;
                        display: flex;
                        align-items: center;
                    }

                    .add-item .checkbox {
                        border: 2px solid #4b5563;
                        background: transparent;
                    }

                    .add-item input {
                        margin: 0;
                        background: transparent;
                        border: none;
                        border-bottom: 1px solid #374151;
                        padding: 8px 12px;
                        color: #f3f4f6;
                        flex: 1;
                        font-size: 16px;
                    }

                    .add-item input:focus {
                        outline: none;
                        border-bottom-color: #6b7280;
                    }

                    .menu {
                        position: absolute;
                        right: 20px;
                        top: 70px;
                        background: #1f2937;
                        border: 1px solid #374151;
                        border-radius: 12px;
                        padding: 8px;
                        min-width: 224px;
                        box-shadow: 0 10px 25px rgba(0,0,0,0.5);
                        z-index: 100;
                    }

                    .menu-item {
                        padding: 12px 16px;
                        cursor: pointer;
                        border-radius: 8px;
                        display: flex;
                        align-items: center;
                        gap: 12px;
                        transition: background 0.2s;
                        font-size: 15px;
                    }

                    .menu-item:hover {
                        background: #374151;
                    }

                    .menu-item.danger {
                        color: #ef4444;
                    }

                    .menu-item svg {
                        width: 20px;
                        height: 20px;
                        flex-shrink: 0;
                    }

                    .modal {
                        position: fixed;
                        inset: 0;
                        background: rgba(0, 0, 0, 0.75);
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        z-index: 200;
                        padding: 24px;
                    }

                    .modal-content {
                        background: #1f2937;
                        border-radius: 16px;
                        padding: 32px;
                        max-width: 384px;
                        width: 100%;
                        border: 1px solid #374151;
                    }

                    .modal-title {
                        text-align: center;
                        margin-bottom: 24px;
                        font-size: 20px;
                        font-weight: 500;
                    }

                    .empty-state {
                        display: flex;
                        flex-direction: column;
                        align-items: center;
                        justify-content: center;
                        min-height: 60vh;
                        text-align: center;
                    }

                    .empty-icon {
                        width: 160px;
                        height: 160px;
                        background: #1f2937;
                        border-radius: 50%;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        margin-bottom: 32px;
                    }

                    .empty-icon svg {
                        width: 80px;
                        height: 80px;
                        color: #4b5563;
                    }

                    .empty-text {
                        color: #6b7280;
                        font-size: 18px;
                        margin-bottom: 32px;
                    }

                    .empty-title {
                        font-size: 24px;
                        font-weight: 500;
                        margin-bottom: 24px;
                    }

                    ::-webkit-scrollbar {
                        width: 8px;
                    }

                    ::-webkit-scrollbar-track {
                        background: #1f2937;
                    }

                    ::-webkit-scrollbar-thumb {
                        background: #4b5563;
                        border-radius: 4px;
                    }
                    "#
                }
            }
            body {
                (content)
            }
            script {
                (PreEscaped(r#"
                // SSE connection for live updates
                let eventSource = null;
                let reconnectAttempts = 0;
                const maxReconnectAttempts = 10;

                function connectSSE() {
                    if (eventSource) {
                        eventSource.close();
                    }

                    eventSource = new EventSource('/events');

                    eventSource.onmessage = function(event) {
                        if (event.data === 'reload') {
                            // Reload current view without full page refresh
                            const currentPath = window.location.pathname;
                            if (currentPath !== '/') {
                                htmx.ajax('GET', currentPath, {
                                    target: 'body',
                                    swap: 'outerHTML'
                                });
                            } else {
                                htmx.ajax('GET', '/', {
                                    target: 'body',
                                    swap: 'outerHTML'
                                });
                            }
                        }
                        reconnectAttempts = 0;
                    };

                    eventSource.onerror = function() {
                        eventSource.close();
                        if (reconnectAttempts < maxReconnectAttempts) {
                            reconnectAttempts++;
                            setTimeout(connectSSE, Math.min(1000 * Math.pow(2, reconnectAttempts), 30000));
                        }
                    };
                }

                // Handle page visibility
                document.addEventListener('visibilitychange', function() {
                    if (!document.hidden) {
                        // Page became visible - reconnect SSE and check for updates
                        connectSSE();
                        const currentPath = window.location.pathname;
                        htmx.ajax('GET', currentPath, {
                            target: 'body',
                            swap: 'outerHTML'
                        });
                    } else {
                        // Page hidden - close SSE to save resources
                        if (eventSource) {
                            eventSource.close();
                        }
                    }
                });

                // Initial connection
                connectSSE();

                window.handleCheckboxClick = function(event, listId, idx) {
                    event.preventDefault();
                    event.stopPropagation();

                    var checkbox = event.target;
                    var item = checkbox.closest('.item');
                    var isCompleted = item.classList.contains('completed');
                    var container = document.querySelector('.container');
                    var isHiding = container && container.getAttribute('data-hide-completed') === 'true';

                    if (!isCompleted && isHiding) {
                        item.classList.add('completed');
                        checkbox.classList.add('checked');

                        anime({
                            targets: item,
                            opacity: [1, 0.3],
                            duration: 3000,
                            easing: 'linear',
                            complete: function() {
                                anime({
                                    targets: item,
                                    opacity: 0,
                                    translateX: -30,
                                    duration: 300,
                                    easing: 'easeInQuad',
                                    complete: function() {
                                        htmx.ajax('POST', '/list/' + listId + '/toggle/' + idx, {
                                            target: 'body',
                                            swap: 'outerHTML'
                                        });
                                    }
                                });
                            }
                        });
                    } else {
                        htmx.ajax('POST', '/list/' + listId + '/toggle/' + idx, {
                            target: 'body',
                            swap: 'outerHTML'
                        });
                    }
                };

                window.editItem = function(element, listId, idx) {
                    var itemText = element.textContent;
                    var input = document.createElement('input');
                    input.type = 'text';
                    input.value = itemText;
                    input.className = 'edit-input';
                    input.style.cssText = 'background: #1f2937; border: 2px solid #2563eb; border-radius: 4px; padding: 4px 8px; color: #f3f4f6; font-size: 16px; flex: 1;';

                    element.parentNode.replaceChild(input, element);
                    input.focus();
                    input.select();

                    function finishEdit() {
                        var newValue = input.value.trim();
                        if (newValue && newValue !== itemText) {
                            htmx.ajax('POST', '/list/' + listId + '/edit/' + idx, {
                                target: 'body',
                                swap: 'outerHTML',
                                values: {item: newValue}
                            });
                        } else {
                            var span = document.createElement('span');
                            span.textContent = itemText;
                            span.className = 'item-text';
                            span.style.flex = '1';
                            span.onclick = function() { window.editItem(span, listId, idx); };
                            input.parentNode.replaceChild(span, input);
                        }
                    }

                    input.addEventListener('blur', finishEdit);
                    input.addEventListener('keypress', function(e) {
                        if (e.key === 'Enter') {
                            e.preventDefault();
                            finishEdit();
                        }
                    });
                };

                window.handleToggleCompleted = function(listId) {
                    var completedItems = document.querySelectorAll('.item.completed');
                    var menuItem = event.target.closest('.menu-item');
                    var isHiding = menuItem.textContent.includes('Hide');

                    document.getElementById('menu').style.display = 'none';

                    if (!isHiding || completedItems.length === 0) {
                        htmx.ajax('POST', '/list/' + listId + '/toggle-completed', {
                            target: 'body',
                            swap: 'outerHTML'
                        });
                        return;
                    }

                    anime({
                        targets: completedItems,
                        opacity: 0,
                        translateX: -30,
                        duration: 500,
                        easing: 'easeInQuad',
                        complete: function() {
                            htmx.ajax('POST', '/list/' + listId + '/toggle-completed', {
                                target: 'body',
                                swap: 'outerHTML'
                            });
                        }
                    });
                };

                window.handleDeleteCompleted = function(listId) {
                    var completedItems = document.querySelectorAll('.item.completed');
                    document.getElementById('menu').style.display = 'none';

                    // Always make the server request to delete ALL completed items (visible and hidden)
                    if (completedItems.length === 0) {
                        // No visible items to animate, just make the request directly
                        htmx.ajax('POST', '/list/' + listId + '/delete-completed', {
                            target: 'body',
                            swap: 'outerHTML'
                        });
                        return;
                    }

                    // Animate visible completed items out, then make request
                    anime({
                        targets: completedItems,
                        opacity: 0,
                        translateX: -30,
                        duration: 500,
                        easing: 'easeInQuad',
                        complete: function() {
                            htmx.ajax('POST', '/list/' + listId + '/delete-completed', {
                                target: 'body',
                                swap: 'outerHTML'
                            });
                        }
                    });
                };

                function initializeSwipes() {
                    document.querySelectorAll('.item').forEach(function(itemElement) {
                        if (itemElement.hammerInitialized) return;
                        itemElement.hammerInitialized = true;

                        var hammer = new Hammer(itemElement);
                        hammer.get('pan').set({ direction: Hammer.DIRECTION_HORIZONTAL, threshold: 10 });

                        var startPos = 0;
                        var currentPos = 0;
                        var isPanning = false;

                        hammer.on('panstart', function(e) {
                            isPanning = true;
                            startPos = 0;
                            itemElement.style.transition = 'none';
                        });

                        hammer.on('panmove', function(e) {
                            if (!isPanning) return;
                            currentPos = e.deltaX;
                            itemElement.style.transform = 'translateX(' + currentPos + 'px)';
                        });

                        hammer.on('panend', function(e) {
                            if (!isPanning) return;
                            isPanning = false;

                            var threshold = 100;
                            var deleteUrl = itemElement.getAttribute('data-delete-url');

                            if (Math.abs(currentPos) > threshold && deleteUrl) {
                                anime({
                                    targets: itemElement,
                                    translateX: currentPos > 0 ? 300 : -300,
                                    opacity: 0,
                                    duration: 300,
                                    easing: 'easeOutQuad',
                                    complete: function() {
                                        htmx.ajax('POST', deleteUrl, {
                                            target: 'body',
                                            swap: 'outerHTML'
                                        });
                                    }
                                });
                            } else {
                                anime({
                                    targets: itemElement,
                                    translateX: 0,
                                    duration: 300,
                                    easing: 'easeOutQuad'
                                });
                            }

                            currentPos = 0;
                        });
                    });
                }

                document.body.addEventListener('htmx:afterSwap', function(e) {
                    var input = document.getElementById('add-input');

                    // Only focus if the swap was triggered by the add item form or initial page load
                    // Check if the detail contains the triggering element info
                    var shouldFocus = false;

                    // Check if it was triggered by the add form submission
                    if (e.detail && e.detail.target) {
                        var targetPath = e.detail.pathInfo ? e.detail.pathInfo.requestPath : '';
                        var xhr = e.detail.xhr;

                        // Focus only if it was an add action or first load
                        if (targetPath && targetPath.includes('/add')) {
                            shouldFocus = true;
                        } else if (!targetPath) {
                            // Initial page load - focus on first visit
                            shouldFocus = true;
                        }
                    }

                    if (input && shouldFocus) {
                        input.focus();
                    }

                    setTimeout(initializeSwipes, 50);
                });

                document.addEventListener('DOMContentLoaded', function() {
                    initializeSwipes();

                    // Focus input on initial page load
                    var input = document.getElementById('add-input');
                    if (input) {
                        input.focus();
                    }
                });
                "#))
            }
        }
    }
}

async fn home(State(ctx): State<AppContext>) -> Html<String> {
    let lists = ctx.state.read().unwrap();

    let content = if lists.is_empty() {
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
    } else {
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
    };

    Html(layout(content).into_string())
}

async fn view_list(Path(id): Path<String>, State(ctx): State<AppContext>) -> impl IntoResponse {
    let lists = ctx.state.read().unwrap();
    let Some(list) = lists.get(&id) else {
        return Html(layout(html! { "List not found" }).into_string());
    };

    let visible_items: Vec<_> = list
        .items
        .iter()
        .enumerate()
        .filter(|(_, item)| list.show_completed || !item.completed)
        .collect();

    let hide_completed_attr = if list.show_completed { "false" } else { "true" };

    let content = html! {
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
                    @if list.show_completed {
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
    };

    Html(layout(content).into_string())
}

async fn new_list_form() -> Html<String> {
    let content = html! {
        div .container {
            h1 { "Name your list" }
            form hx-post="/create" hx-target="body" {
                input type="text" name="name" placeholder="Enter grocery list name" required autofocus;
                button .btn.btn-primary type="submit" { "Create grocery list" }
            }
        }
    };

    Html(layout(content).into_string())
}

async fn lists_view(State(ctx): State<AppContext>) -> Markup {
    let lists = ctx.state.read().unwrap();

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
        }
        a .fab href="/new" { "+" }
    }
}

#[derive(Deserialize)]
struct CreateForm {
    name: String,
}

async fn create_list(
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

    Html(layout(lists_view(State(ctx)).await).into_string())
}

#[derive(Deserialize)]
struct AddItemForm {
    item: String,
}

async fn add_item(
    Path(id): Path<String>,
    State(ctx): State<AppContext>,
    Form(form): Form<AddItemForm>,
) -> impl IntoResponse {
    if let Some(list) = ctx.state.write().unwrap().get_mut(&id) {
        list.items.push(Item {
            name: form.item,
            completed: false,
        });
    }
    save_data(&ctx.state).await;
    view_list(Path(id), State(ctx)).await
}

async fn edit_item(
    Path((id, idx)): Path<(String, usize)>,
    State(ctx): State<AppContext>,
    Form(form): Form<AddItemForm>,
) -> impl IntoResponse {
    if let Some(list) = ctx.state.write().unwrap().get_mut(&id) {
        if let Some(item) = list.items.get_mut(idx) {
            item.name = form.item;
        }
    }
    save_data(&ctx.state).await;
    view_list(Path(id), State(ctx)).await
}

async fn delete_item(
    Path((id, idx)): Path<(String, usize)>,
    State(ctx): State<AppContext>,
) -> impl IntoResponse {
    if let Some(list) = ctx.state.write().unwrap().get_mut(&id) {
        if idx < list.items.len() {
            list.items.remove(idx);
        }
    }
    save_data(&ctx.state).await;
    view_list(Path(id), State(ctx)).await
}

async fn toggle_item(
    Path((id, idx)): Path<(String, usize)>,
    State(ctx): State<AppContext>,
) -> impl IntoResponse {
    if let Some(list) = ctx.state.write().unwrap().get_mut(&id) {
        if let Some(item) = list.items.get_mut(idx) {
            item.completed = !item.completed;
        }
    }
    save_data(&ctx.state).await;
    view_list(Path(id), State(ctx)).await
}

async fn toggle_show_completed(
    Path(id): Path<String>,
    State(ctx): State<AppContext>,
) -> impl IntoResponse {
    if let Some(list) = ctx.state.write().unwrap().get_mut(&id) {
        list.show_completed = !list.show_completed;
    }
    save_data(&ctx.state).await;
    view_list(Path(id), State(ctx)).await
}

async fn delete_completed_items(
    Path(id): Path<String>,
    State(ctx): State<AppContext>,
) -> impl IntoResponse {
    if let Some(list) = ctx.state.write().unwrap().get_mut(&id) {
        list.items.retain(|item| !item.completed);
    }
    save_data(&ctx.state).await;
    view_list(Path(id), State(ctx)).await
}

async fn sort_list(Path(id): Path<String>, State(ctx): State<AppContext>) -> impl IntoResponse {
    if let Some(list) = ctx.state.write().unwrap().get_mut(&id) {
        list.items
            .sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    }
    save_data(&ctx.state).await;
    view_list(Path(id), State(ctx)).await
}

async fn delete_list(Path(id): Path<String>, State(ctx): State<AppContext>) -> impl IntoResponse {
    ctx.state.write().unwrap().remove(&id);
    save_data(&ctx.state).await;
    Html(layout(lists_view(State(ctx)).await).into_string())
}
