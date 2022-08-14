use chrono::{DateTime, Local, ParseError};

const LAYOUT: &'static str = "%Y-%m-%d %H:%M:%S";

//get current datetime
pub fn current_dt() -> (DateTime<Local>, String) {
    let now = Local::now();
    let dt = now.format(LAYOUT).to_string();
    (now, dt)
}

//dt to timestamp
pub fn dt_to_timestamp(dt: DateTime<Local>) -> i64 {
    dt.timestamp()
}

//str to dt
pub fn str_to_dt(dtstr: &str) -> Result<DateTime<Local>, ParseError> {
    let dt = dtstr.parse::<DateTime<Local>>();
    dt
}
