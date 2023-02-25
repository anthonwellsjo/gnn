use arw_brr::get_app_path;
use rusqlite::{Connection, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AuthRequest {
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
}

/// All db methonds for related to authentication
pub struct Auth;

impl Auth {
    pub fn save_token(session: AuthRequest) -> Result<AuthRequest> {
        println!("save token");

        let conn = Self::get_db_connection()?;

        conn.execute(
            "INSERT INTO auth (access_token, token_type, scope) values (?1, ?2, ?3)",
            &[&session.access_token, &session.token_type, &session.scope],
        )?;

        conn.close()
            .unwrap_or_else(|_| panic!("Panicking while closing conection."));

        Ok(session)
    }

    ///  Gets connection to DB. This function will create a new DB if
    ///  not already present
    pub fn get_db_connection() -> Result<Connection> {
        println!("get db connection");

        let conn = Connection::open(get_app_path("gnn"))?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS auth (
             id INTEGER PRIMARY KEY,
             access_token TEXT NOT NULL,
             token_type TEXT NOT NULL,
             scope TEXT NOT NULL,
             created_at TEXT DEFAULT CURRENT_TIMESTAMP
         )",
            [],
        )?;
        Ok(conn)
    }

    /// Gets the latest used token
    pub fn get_last_session() -> Result<Option<AuthRequest>> {
        let conn = Self::get_db_connection()?;

        let mut stmt = conn.prepare(
            "SELECT access_token, token_type, scope, MAX(created_at)
             FROM auth
             LIMIT 1",
        )?;

        let auth_map = stmt.query_map([], |row| {
            Ok(AuthRequest {
                access_token: row.get(0)?,
                token_type: row.get(1)?,
                scope: row.get(2)?,
            })
        })?;

        let auth = auth_map.into_iter().next().unwrap()?;

        //If no token, return none
        if auth.access_token.is_none() {
            return Ok(None);
        }
        Ok(Some(auth))
    }
}

#[derive(Deserialize, Debug, Clone )]
pub struct User {
    name: Option<String>,
    login: Option<String>,
    avatar_url: Option<String>,
    html_url: Option<String>,
    subscriptions_url: Option<String>,
    organizations_url: Option<String>,
    repos_url: Option<String>,
    events_url: Option<String>,
    received_events_url: Option<String>,
}

impl User {
    pub fn save(user: &Self) -> Result<()> {
        println!("save token");

        let conn = Self::get_db_connection()?;

        conn.execute(
            "INSERT INTO users (name, login, avatar_url, html_url, subscriptions_url, organizations_url, repos_url, events_url, received_events_url) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            &[&user.name, &user.login, &user.avatar_url, &user.html_url, &user.subscriptions_url, &user.organizations_url, &user.repos_url, &user.events_url, &user.received_events_url],
        )?;

        conn.close()
            .unwrap_or_else(|_| panic!("Panicking while closing conection."));

        Ok(())
    }

    ///  Gets connection to DB. This function will create a new DB if
    ///  not already present
    pub fn get_db_connection() -> Result<Connection> {
        println!("get db connection");

        let conn = Connection::open(get_app_path("gnn"))?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            name TEXT,
            login TEXT,
            avatar_url TEXT,
            html_url: TEXT,
            subscriptions_url: TEXT,
            organizations_url: TEXT,
            repos_url: TEXT,
            events_url: TEXT,
            received_events_url: TEXT
         )",
            [],
        )?;
        Ok(conn)
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

#[derive(Deserialize, Debug, Clone)]
pub struct Repository {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub owner: Option<User>,
    pub private: Option<bool>,
    pub html_url: Option<String>,
    pub description: Option<String>,
    pub fork: Option<bool>,
    pub url: Option<String>,
}

#[derive(Deserialize, Debug, Clone )]
pub struct NotificationSubject {
    pub title: Option<String>,
    pub url: Option<String>,
    pub latest_comment_url: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Notification {
    pub id: Option<String>,
    pub repository: Option<Repository>,
    pub subject: Option<NotificationSubject>,
    pub reason: Option<String>,
    pub unread: Option<bool>,
    pub updated_at: Option<String>,
    pub last_read_at: Option<String>,
    pub url: Option<String>,
    pub subscription_url: Option<String>,
}

impl Notification {
    pub async fn save(notification: &Self) -> Result<()> {
        println!("save notification");

        let conn = Self::get_db_connection()?;

        conn.execute(
            "INSERT INTO notification (id, url) values (?1, ?2)",
            &[&Notification::get_spec_id(notification.id.as_ref().unwrap()), &notification.url.as_ref().unwrap()],
        )?;

        conn.close()
            .unwrap_or_else(|_| panic!("Panicking while closing conection."));

        Ok(())
    }

    pub async fn save_many(arg: &mut crate::app::Session, notifications: Vec<Notification>) {
        for no in notifications{
            Self::save(&no).await;
        }
    }

    pub fn get_db_connection() -> Result<Connection> {
        let conn = Connection::open(get_app_path("gnn"))?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS notification (
            id TEXT,
            url: TEXT
         )",
            [],
        )?;
        Ok(conn)
    }

    pub fn clean_table() -> Result<()> {
        let conn = Connection::open(get_app_path("gnn"))?;
        conn.execute(
            "DELETE * FROM notification",
            [],
        )?;
        Ok(())
    }

    pub fn get_by_id(id: String) -> Result<Option<Vec<Notification>>>{
        let conn = Connection::open(get_app_path("gnn"))?;

        let mut stmt = conn.prepare(
            &("SELECT (id, url)
             FROM notification 
             WHERE id=".to_owned() +&id),
        )?;

        let notifications = stmt.query_map([], |row| {
            Ok(Notification {
                id: row.get(0)?,
                url: row.get(1)?,
                subscription_url: None,
                reason: None,
                updated_at: None,
                subject: None,
                unread: None,
                repository: None,
                last_read_at: None
            })
        })?;

        let mut nots: Vec<Notification> = Vec::new();

        for not in notifications {
            let not = match not {
                Ok(res) => res,
                Err(error) => panic!("Problem opening the file: {:?}", error),
            };
            nots.push(not);
        }

        Ok(Some(nots))

    }

}


