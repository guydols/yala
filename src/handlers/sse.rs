use crate::state::AppContext;
use axum::{
    extract::State,
    response::{Sse, sse::Event},
};
use futures::stream::{Stream, StreamExt};
use std::{convert::Infallible, time::Duration};
use tokio_stream::wrappers::BroadcastStream;

pub async fn sse_handler(
    State(ctx): State<AppContext>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = ctx.update_tx.subscribe();
    let stream = BroadcastStream::new(rx).map(|result| match result {
        Ok(data) => Ok(Event::default().data(data)),
        Err(_) => Ok(Event::default().data("")),
    });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(30))
            .text("keep-alive"),
    )
}
