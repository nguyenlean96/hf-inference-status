use leptos::prelude::*;
use std::time::Duration;

/// Hook to wrap the underlying `setInterval` call and make it reactive w.r.t.
/// possible changes of the timer interval.
pub fn use_interval<T, F>(
    interval_millis: T,
    f: F,
) -> (ReadSignal<bool>, impl Fn() + Clone, impl Fn() + Clone)
where
    F: Fn() + Clone + 'static,
    T: Into<Signal<u64>> + 'static,
{
    let interval_millis = interval_millis.into();
    let (active, set_active) = signal(true);

    Effect::new(move |prev_handle: Option<Option<IntervalHandle>>| {
        // Effects get their previous return value as an argument.
        // Each time the effect runs, it will return the interval handle,
        // so if we have a previous one, we cancel it.
        if let Some(Some(prev_handle)) = prev_handle {
            prev_handle.clear();
        }

        // Here, we return the handle.
        if active.get() {
            let handle = set_interval_with_handle(
                f.clone(),
                // This is the only reactive access, so this effect will only
                // re-run when the interval changes.
                Duration::from_millis(interval_millis.get()),
            )
            .expect("could not create interval");

            Some(handle)
        } else {
            None
        }
    });

    (
        active,
        move || set_active.set(true),
        move || set_active.set(false),
    )
}
