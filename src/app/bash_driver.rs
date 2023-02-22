use crate::{app::{ActionResponse, ActionResponseType}, db::{Notification, get_notification_reason}};

pub fn display_action_response(res: &ActionResponse) {
    display_icon(&res.res_type);

    if res.message.len() > 0 {
        println!(" {}", res.message);
    }

    if res.res_type == ActionResponseType::Content {
        show_notifications(&res.notifications);
    }
}

fn show_notifications(not: &Option<Vec<Notification>>){
    let not = not.as_ref().unwrap();
    for no in not.into_iter() {
        println!("{:?} {:?}", get_notification_reason(no.reason.as_ref().unwrap()), no.subject.as_ref().unwrap().r#type.as_ref().unwrap());

    }

}

fn display_icon(res_type: &ActionResponseType){
    match res_type{
        ActionResponseType::Error => print!("❌ "),
        ActionResponseType::Success => print!("👍 "),
        ActionResponseType::Silent => {},
        ActionResponseType::Content => {}
        }
}

