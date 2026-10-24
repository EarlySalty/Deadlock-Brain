use sqlx::PgConnection;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

const CAPACITY: usize = 2048;

#[derive(Clone)]
pub(super) enum Rendered {
    Page(Arc<str>),
    Redirect(Arc<str>),
}

#[derive(Default)]
struct State {
    generation: String,
    pages: HashMap<String, Rendered>,
}

#[derive(Clone, Default)]
pub(super) struct PageCache {
    state: Arc<Mutex<State>>,
}

impl PageCache {
    fn state(&self) -> MutexGuard<'_, State> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub(super) fn get(&self, generation: &str, key: &str) -> Option<Rendered> {
        let state = self.state();
        (state.generation == generation)
            .then(|| state.pages.get(key).cloned())
            .flatten()
    }

    pub(super) fn put(&self, generation: &str, key: String, rendered: Rendered) {
        let mut state = self.state();
        if state.generation != generation {
            state.generation = generation.to_owned();
            state.pages.clear();
        }
        if state.pages.len() < CAPACITY || state.pages.contains_key(&key) {
            state.pages.insert(key, rendered);
        }
    }
}

pub(super) async fn generation(connection: &mut PgConnection) -> sqlx::Result<String> {
    sqlx::query_scalar("SELECT serve.site_generation()")
        .fetch_one(connection)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page(html: &str) -> Rendered {
        Rendered::Page(Arc::from(html))
    }

    fn html(rendered: Option<Rendered>) -> Option<String> {
        match rendered {
            Some(Rendered::Page(html)) => Some(html.to_string()),
            _ => None,
        }
    }

    #[test]
    fn pages_are_served_only_for_their_data_generation() {
        let cache = PageCache::default();
        cache.put("g1", "hero/a".into(), page("alt"));
        assert_eq!(html(cache.get("g1", "hero/a")).as_deref(), Some("alt"));
        assert!(cache.get("g2", "hero/a").is_none());
        cache.put("g2", "hero/b".into(), page("neu"));
        assert!(cache.get("g2", "hero/a").is_none());
        assert!(cache.get("g1", "hero/b").is_none());
        assert_eq!(html(cache.get("g2", "hero/b")).as_deref(), Some("neu"));
    }

    #[test]
    fn capacity_bounds_distinct_entries() {
        let cache = PageCache::default();
        for index in 0..CAPACITY + 10 {
            cache.put("g", format!("item/{index}"), page("x"));
        }
        assert_eq!(cache.state().pages.len(), CAPACITY);
        assert!(cache.get("g", &format!("item/{}", CAPACITY + 5)).is_none());
    }
}
