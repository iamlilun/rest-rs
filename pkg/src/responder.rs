use serde::Serialize;

use std::fmt;
use std::marker::PhantomData;

pub trait Data: Serialize + Sized {}

#[derive(Serialize)]
pub struct Detail(pub String);
impl Data for Detail {}

/**
 * content struct
 */
#[derive(Serialize)]
pub struct Content<D> {
    pub status: i32,
    pub msg: String,
    pub data: D,
}

/**
 * paginate struct
 */
#[derive(Serialize)]
pub struct Paginate<C> {
    #[serde(flatten)]
    pub content: C,
    pub total: i32,
    pub per_page: i32,
    pub corrent_page: i32,
}

/**
 * make success resp data
 */
pub fn success<D: Data>(data: D) -> (i32, Content<D>) {
    let status = to_code(&StatusCode::STATUS_OK);
    let cnt = Content {
        status,
        msg: StatusCode::STATUS_OK.to_string(),
        data,
    };

    (status, cnt)
}

/**
 * make failed resp data
 */
pub fn failed<D: Data>(status: StatusCode, data: D) -> (i32, Content<D>) {
    let status = to_code(&status);
    let cnt = Content {
        status: status,
        msg: status.to_string(),
        data,
    };

    (status, cnt)
}

/**
 * make pagin resp data
 */
pub fn pagination(
    data: impl Data,
    page: i32,
    size: i32,
    total: i32,
) -> (i32, Paginate<Content<impl Data>>) {
    let status = to_code(&StatusCode::STATUS_OK);
    let cnt = Content {
        status: status,
        msg: StatusCode::STATUS_OK.to_string(),
        data,
    };

    let pagin = Paginate {
        content: cnt,
        total: total,
        per_page: size,
        corrent_page: page,
    };

    (status, pagin)
}

/**
 * status code enum
 */
#[derive(Copy, Clone)]
pub enum StatusCode {
    STATUS_OK = 2000,
    STATUS_BADREQ = 4000,
    STATUS_VALIDATION = 4001,
    STATUS_DUPLICATE = 4002,
    STATUS_FORBIDDEN = 4003,
    STATUS_NOT_FOUND = 4004,
    STATUS_INTERNAL = 5000,
    STATUS_UNKNOWNERR = 5001,
}

/**
 * enum to code number
 */
fn to_code(status: &StatusCode) -> i32 {
    *status as i32
}

/**
 * support to_string()
 */
impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StatusCode::STATUS_OK => write!(f, "Ok"),
            StatusCode::STATUS_BADREQ => write!(f, "Bad request"),
            StatusCode::STATUS_VALIDATION => write!(f, "Validation failed"),
            StatusCode::STATUS_DUPLICATE => write!(f, "Already exists"),
            StatusCode::STATUS_FORBIDDEN => write!(f, "Forbidden"),
            StatusCode::STATUS_NOT_FOUND => write!(f, "Resource not found"),
            StatusCode::STATUS_INTERNAL => write!(f, "Internal error"),
            StatusCode::STATUS_UNKNOWNERR => write!(f, "Unknown error"),
        }
    }
}
