use std::cell::RefCell;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TraceRequest {
    pub question: String,
    pub history: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotAnswered {
    pub stage: String,
    pub reason: String,
}

thread_local! {
    static REQUEST: RefCell<Option<TraceRequest>> = const { RefCell::new(None) };
    static NOT_ANSWERED: RefCell<Option<NotAnswered>> = const { RefCell::new(None) };
}

struct Restore<T: 'static> {
    slot: &'static std::thread::LocalKey<RefCell<Option<T>>>,
    previous: Option<T>,
}

impl<T: 'static> Drop for Restore<T> {
    fn drop(&mut self) {
        let previous = self.previous.take();
        self.slot.with(|slot| {
            slot.replace(previous);
        });
    }
}

pub fn with_request<R>(request: TraceRequest, action: impl FnOnce() -> R) -> R {
    let _restore = Restore {
        slot: &REQUEST,
        previous: REQUEST.with(|slot| slot.replace(Some(request))),
    };
    action()
}

pub fn current_request() -> Option<TraceRequest> {
    REQUEST.with(|slot| slot.borrow().clone())
}

pub fn note_not_answered(stage: &str, reason: &str) {
    NOT_ANSWERED.with(|slot| {
        slot.replace(Some(NotAnswered {
            stage: stage.into(),
            reason: reason.into(),
        }));
    });
}

pub fn observe_not_answered<R>(action: impl FnOnce() -> R) -> (R, Option<NotAnswered>) {
    let restore = Restore {
        slot: &NOT_ANSWERED,
        previous: NOT_ANSWERED.with(|slot| slot.replace(None)),
    };
    let result = action();
    let observed = NOT_ANSWERED.with(|slot| slot.borrow_mut().take());
    drop(restore);
    (result, observed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_and_outcome_are_scoped_and_restored() {
        assert!(current_request().is_none());
        let inner = with_request(
            TraceRequest {
                question: "Frage".into(),
                history: vec!["Vorher".into()],
            },
            || {
                let (nested, observed) = observe_not_answered(|| {
                    note_not_answered("final_validation", "value_gate_rejected");
                    current_request()
                });
                (nested, observed)
            },
        );
        assert_eq!(
            inner.0.map(|request| request.history),
            Some(vec!["Vorher".to_string()])
        );
        assert_eq!(
            inner.1,
            Some(NotAnswered {
                stage: "final_validation".into(),
                reason: "value_gate_rejected".into()
            })
        );
        assert!(current_request().is_none());
        let (_, outer) = observe_not_answered(|| {
            note_not_answered("a", "b");
            let (_, nested) = observe_not_answered(|| ());
            assert!(nested.is_none());
        });
        assert_eq!(outer.map(|note| note.stage), Some("a".to_string()));
    }
}
