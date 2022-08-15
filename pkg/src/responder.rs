use serde::Serialize;

use std::fmt;
// use std::marker::PhantomData;

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
    let status = StatusCode::StatusOK;
    let cnt = Content {
        status: status.to_int(),
        msg: status.to_string(),
        data,
    };

    (status.to_int(), cnt)
}

/**
 * make failed resp data
 */
pub fn failed<D: Data>(status: StatusCode, data: D) -> (i32, Content<D>) {
    let cnt = Content {
        status: status.to_int(),
        msg: status.to_string(),
        data,
    };

    (status.to_int(), cnt)
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
    let status = StatusCode::StatusOK;
    let cnt = Content {
        status: status.to_int(),
        msg: status.to_string(),
        data,
    };

    let pagin = Paginate {
        content: cnt,
        total: total,
        per_page: size,
        corrent_page: page,
    };

    (status.to_int(), pagin)
}

/**
 * status code enum
 */
#[derive(Copy, Clone)]
pub enum StatusCode {
    StatusOK = 2000,
    StatusBadReq = 4000,
    StatusValidation = 4001,
    StatusDuplicate = 4002,
    StatusForbidden = 4003,
    StatusNotFound = 4004,
    StatusInternal = 5000,
    StatusUnknownErr = 5001,
}

impl StatusCode {
    pub fn to_int(&self) -> i32 {
        *self as i32
    }
}

/**
 * support to_string()
 */
impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StatusCode::StatusOK => write!(f, "Ok"),
            StatusCode::StatusBadReq => write!(f, "Bad request"),
            StatusCode::StatusValidation => write!(f, "Validation failed"),
            StatusCode::StatusDuplicate => write!(f, "Already exists"),
            StatusCode::StatusForbidden => write!(f, "Forbidden"),
            StatusCode::StatusNotFound => write!(f, "Resource not found"),
            StatusCode::StatusInternal => write!(f, "Internal error"),
            StatusCode::StatusUnknownErr => write!(f, "Unknown error"),
        }
    }
}
