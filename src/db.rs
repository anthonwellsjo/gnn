use std::error::Error;

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

#[derive(Deserialize, Debug)]
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


pub struct Repository{
      id: Option<i64>,
      name: Option<String>,
      owner: User,
      private: Option<bool>,
      html_url: Option<String>,
      description: Option<String>,
      fork: Option<bool>,
      url: Option<String>
}

pub struct NotificationSubject{      "title": "Greetings",
      url: Option<String>,
      latest_comment_url: Option<String>,
      r#type: Option<String>
}

pub struct Notification{
    repository: Repository,
    subject: {
    },
    "reason": "subscribed",
    "unread": true,
    "updated_at": "2014-11-07T22:01:45Z",
    "last_read_at": "2014-11-07T22:01:45Z",
    "url": "https://api.github.com/notifications/threads/1",
    "subscription_url": "https://api.github.com/notifications/threads/1/subscription"

}

impl Notification {
}
