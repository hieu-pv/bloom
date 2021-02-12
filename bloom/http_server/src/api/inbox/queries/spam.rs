use crate::{
    api::inbox::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::FindSpamInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn spam(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::GetSpam>,
    actor: Actor,
) -> Result<api::Response<model::Inbox>, kernel::Error> {
    let input = input.into_inner();
    let service_input = FindSpamInput {
        namespace_id: input.namespace_id,
    };
    let conversations = ctx.inbox_service.find_spam(actor, service_input).await?;

    let inbox = model::Inbox {
        conversations: conversations.into_iter().map(Into::into).collect(),
    };
    Ok(api::Response::ok(inbox))
}
