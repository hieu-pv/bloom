use crate::repository::Repository;
use kernel::{db::DB, drivers};
use std::sync::Arc;

mod create_contact;
mod create_newsletter_list;
mod create_newsletter_message;
mod delete_contact;
mod delete_newsletter_list;
mod delete_newsletter_message;
mod import_contacts;
mod send_newsletter_message;

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: DB,
    kernel_service: Arc<kernel::Service>,
    queue: Arc<dyn drivers::Queue>,
}

impl Service {
    pub fn new(kernel_service: Arc<kernel::Service>, db: DB, queue: Arc<dyn drivers::Queue>) -> Service {
        let repo = Repository::new();

        Service {
            db,
            repo,
            kernel_service,
            queue,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateContactInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct CreateNewsletterListInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct CreateNewsletterMessageInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct DeleteContactInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct DeleteNewsletterListInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct DeleteNewsletterMessageInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct ImportContactsInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct SendNewsletterMessageInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct SendTestNewsletterMessageInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct UpdateNewsletterListInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct UpdateNewsletterMessageInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct FindContactInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct FindContactsInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct FindNewsletterListInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct FindNewsletterListsInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct FindNewsletterMessageInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct FindNewsletterMessagesInput {
    // TODO
}
