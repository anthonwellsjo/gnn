use chrono::DateTime;
use std::cmp::Ordering;

use crate::{
    app::{ActionResponse, ActionResponseType},
    models::{Notification, Thread},
};

pub fn display_action_response(res: &ActionResponse) {
    display_icon(&res.res_type);

    if res.message.len() > 0 {
        println!(" {}", res.message);
    }

    if res.res_type == ActionResponseType::Content {
        let content = res.content.as_ref().unwrap();

        if content.notifications.is_some(){
            show_notifications(&content.notifications.as_ref().unwrap());
        }

        if content.thread.is_some(){
            show_thread(&content.thread.as_ref().unwrap());
        }
    }
}

fn show_thread(t: &Thread) {
    println!("Reason: {:?}", get_notification_reason(&t.reason));
    println!("Repo name: {:?}", &t.repository.name);
    println!("Owner: {:?}", &t.repository.owner.login);
}

fn show_notifications(not: &Vec<Notification>) {

    println!(
        "  {} {} {} | {} | {} | {}",
        "",
        format_text(&3, &"id".to_owned(), true),
        format_text(&8, &" repo".to_owned(), false),
        format_text(&10, &"  type".to_owned(), false),
        format_text(&25, &"  subject".to_owned(), false),
        "  "
    );
    println!("--------------------------------------------------------------------");

    for no in not.into_iter() {
        let date = DateTime::parse_from_rfc3339(&no.updated_at).unwrap();
        println!(
            "{}  {} {} | {} | {} | {} {}",
            get_unread_icon(no.unread),
            format_text(&3, &Notification::get_short_id(&no.id), true),
            format_text(&8, &no.repository.name, false),
            format_text(&10, &no.subject.type_field, false),
            format_text(&25, &no.subject.title, false),
            date.format("%d/%m %H:%M"),
            format_text(&3, &get_notification_reason_icon(&no.reason), true),
        );
    }
}

fn format_text(n_of_chars: &usize, text: &std::string::String, no_wrap: bool) -> String {
    let mut was_too_long = false;
    let mut text: String = text.to_owned();

    loop {
        match text.chars().count().cmp(n_of_chars) {
            Ordering::Less => {
                text = text.to_owned() + " ";
            }
            Ordering::Equal => {
                break;
            }
            Ordering::Greater => {
                text.pop().unwrap();
                was_too_long = true;
            }
        }
    }
    if was_too_long && !no_wrap {
        text = text.to_owned() + "..";
    } else {
        text = text.to_owned() + "  ";
    }
    return text.to_owned();
}

fn display_icon(res_type: &ActionResponseType) {
    match res_type {
        ActionResponseType::Error => print!("❌ "),
        ActionResponseType::Success => print!("👍 "),
        ActionResponseType::Silent => {}
        ActionResponseType::Content => {}
    }
}

fn get_unread_icon(unread: bool) -> String {
    if unread {
        return " ".to_owned();
    } else {
        return "👀".to_owned();
    }
}

fn get_notification_reason_icon(s: &str) -> String {
    match s {
        "assign" => "📋".to_owned(),
        "author" => "🤴".to_owned(),
        "comment" => "🗣".to_owned(),
        "ci_activity" => "🔁 ".to_owned(),
        "invitation" => "🍴 ".to_owned(),
        "manual" => "𝌘 ".to_owned(),
        "mention" => "🗣".to_owned(),
        "review_requested" => "👨‍🏫".to_owned(),
        "security_alert" => "⚠️".to_owned(),
        "state_change" => "🏁 ".to_owned(),
        "subscribed" => "🖋️".to_owned(),
        "team_mention" => "🗣".to_owned(),
        &_ => "Unregistered reason".to_owned(),
    }
}

pub fn get_notification_reason(s: &str) -> String {
    match s{
            "assign" => "You were assigned to the issue.".to_owned(),
            "author" => "You created the thread.".to_owned(),
            "comment" => "You commented on the thread.".to_owned(),
            "ci_activity" => "A GitHub Actions workflow run that you triggered was completed.".to_owned(), 
            "invitation" => "You accepted an invitation to contribute to the repository.".to_owned(),
            "manual" => "You subscribed to the thread (via an issue or pull request).".to_owned(),
            "mention" => "You were specifically @mentioned in the content.".to_owned(),
            "review_requested" => "You, or a team you're a member of, were requested to review a pull request.".to_owned(),
            "security_alert" => "GitHub discovered a security vulnerability in your repository.".to_owned(),
            "state_change" => "You changed the thread state (for example, closing an issue or merging a pull request).".to_owned(),
            "subscribed" => "You're watching the repository.".to_owned(),
            "team_mention" => "You were on a team that was mentioned.".to_owned(),
            &_ => "Unregistered reason".to_owned()
        }
}

