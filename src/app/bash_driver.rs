use crate::{
    app::{ActionResponse, ActionResponseType},
    db::{get_notification_reason, Notification},
};

pub fn display_action_response(res: &ActionResponse) {
    display_icon(&res.res_type);

    if res.message.len() > 0 {
        println!(" {}", res.message);
    }

    if res.res_type == ActionResponseType::Content {
        show_notifications(&res.notifications);
    }
}

fn show_notifications(not: &Option<Vec<Notification>>) {
    let not = not.as_ref().unwrap();
    for no in not.into_iter() {
        println!(
            "{} {} | {} | {} | {}",
            get_unread_icon(*no.unread.as_ref().unwrap()),
            no.repository.as_ref().unwrap().name.as_ref().unwrap(),
            no.subject.as_ref().unwrap().r#type.as_ref().unwrap(),
            no.subject.as_ref().unwrap().title.as_ref().unwrap(),
            get_notification_reason(no.reason.as_ref().unwrap()),
        );
    }
}

fn format_text(n_of_chars: String) -> String{

}

fn display_icon(res_type: &ActionResponseType) {
    match res_type {
        ActionResponseType::Error => print!("❌ "),
        ActionResponseType::Success => print!("👍 "),
        ActionResponseType::Silent => {}
        ActionResponseType::Content => {}
    }
}

fn get_unread_icon(unread: bool) -> String{

    if unread {
        return " ".to_owned()
    } else {
        return "✔".to_owned()
    }
}

fn get_notification_reason_icon(s: &str) -> String {
    match s {
        "assign" => "📋".to_owned(),
        "author" => "🤴".to_owned(),
        "comment" => "🗣".to_owned(),
        "ci_activity" => "▶️ ".to_owned(),
        "invitation" => "🍴 ".to_owned(),
        "manual" => "𝌘 ".to_owned(),
        "mention" => "🗣".to_owned(),
        "review_requested" => "👨‍🏫".to_owned(),
        "security_alert" => "⚠️".to_owned(),
        "state_change" => "🏁 ".to_owned(),
        "subscribed" => "📥 ".to_owned(),
        "team_mention" => "🗣".to_owned(),
        &_ => "Unregistered reason".to_owned(),
    }
}
