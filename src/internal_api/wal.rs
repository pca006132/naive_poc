use super::UserId;
use serde::Serialize;
use serde_json::to_string;
use std::sync::Mutex;

pub trait LogStore {
    // this function has to respect order:
    // if record(r1) happens before (and ends before) record(r2),
    // r1 should appear earlier in the record than r2
    fn record<T: Serialize>(&self, user: UserId, api_name: &str, payload: &T)
        -> Result<(), String>;
}

#[derive(Debug)]
pub struct NaiveLogStore(Mutex<Vec<(UserId, String, String)>>);

impl LogStore for NaiveLogStore {
    fn record<T: Serialize>(
        &self,
        user: UserId,
        api_name: &str,
        payload: &T,
    ) -> Result<(), String> {
        let mut store = self.0.lock().map_err(|_| "Poison".to_owned())?;
        store.push((
            user,
            api_name.into(),
            to_string(payload).map_err(|e| e.to_string())?,
        ));
        Ok(())
    }
}
