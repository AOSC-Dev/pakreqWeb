#![allow(unused)]

use diesel::prelude::*;
use chrono::NaiveDate;
use serde::Serialize;

// Generated by diesel_ext

#[derive(Queryable, Debug)]
pub struct Oauth {
    pub uid: i64,
    pub type_: String,
    pub oid: Option<String>,
    pub token: Option<String>,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Request {
    pub id: i64,
    pub status: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    pub description: Option<String>,
    pub requester_id: i64,
    pub packager_id: Option<i64>,
    pub pub_date: NaiveDate,
    pub note: Option<String>,
}

#[derive(Queryable, Debug, Serialize)]
pub struct RequestStr {
    pub id: i64,
    pub status: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    pub description: Option<String>,
    pub requester: String,
    pub packager: Option<String>,
    pub pub_date: NaiveDate,
    pub note: Option<String>,
}

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing)]
    pub admin: bool,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
}
