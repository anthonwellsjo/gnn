use arw_brr::get_app_path;
use rusqlite::{Connection, Result};
use serde::Deserialize;

use crate::models::{Notification, LocalNotification, User};


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
            .unwrap_or_else(|_| panic!("Panicking while closing connection."));

        Ok(session)
    }

    ///  Gets connection to DB. This function will create a new DB if
    ///  not already present
    pub fn get_db_connection() -> Result<Connection> {
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


impl User {
    pub fn save(user: &Self) -> Result<()> {
        println!("save token");

        let conn = Self::get_db_connection()?;

        conn.execute(
            "INSERT INTO users (name, login, avatar_url, html_url, subscriptions_url, organizations_url, repos_url, events_url, received_events_url) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            &[&user.name, &user.login, &user.avatar_url, &user.html_url, &user.subscriptions_url, &user.organizations_url, &user.repos_url, &user.events_url, &user.received_events_url],
        )?;

        conn.close()
            .unwrap_or_else(|_| panic!("Panicking while closing connection."));

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

impl Notification {
    pub async fn save(notification: &Self) -> Result<()> {
        println!("save notification {:?}", notification);

        let conn = Self::get_db_connection()?;

        println!("execute");
        conn.execute(
            "INSERT INTO notification (gh_id, short_id, url) values (?1, ?2, ?3)",
            &[
                &notification.id,
                &Notification::get_short_id(&notification.id),
                &notification.url,
            ])?;

        conn.close()
            .unwrap_or_else(|_| panic!("Panicking while closing connection."));

        Ok(())
    }

    pub async fn save_many(_arg: &mut crate::app::Session, notifications: Vec<Notification>) {
        for no in notifications {
            Self::save(&no).await.expect("Couldn't save notification");
        }
    }

    pub fn get_db_connection() -> Result<Connection> {
        println!("get conn");
        let conn = Connection::open(get_app_path("gnn"))?;
        println!("got conn");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS notification (
            id INTEGER PRIMARY KEY,
            gh_id TEXT NOT NULL,
            short_id TEXT NOT NULL,
            url TEXT NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
         )",
            [],
        )?;
        println!("created table");
        Ok(conn)
    }

    pub fn clean_table() -> Result<()> {
        let conn = Connection::open(get_app_path("gnn"))?;
        conn.execute("DELETE * FROM notification", [])?;
        Ok(())
    }

    pub fn get_by_id(id: String) -> Result<Option<Vec<LocalNotification>>> {
        let conn = Connection::open(get_app_path("gnn"))?;

        let mut stmt = conn.prepare(
            &("SELECT short_id, url
            FROM notification 
            WHERE short_id="
               .to_owned()
               + &id + "
            LIMIT 1"),
             
        )?;

        let notifications = stmt.query_map([], |row| {
            Ok(LocalNotification {
                short_id: row.get(0)?,
                url: row.get(1)?,
            })
        })?;

        let mut nots: Vec<LocalNotification> = Vec::new();

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
