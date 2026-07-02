#[macro_export]
macro_rules! spawn_task {
    ($($body:tt)*) => {
        tokio::spawn(async move {
            $($body)*
        });
    };
}

#[macro_export]
macro_rules! mutate_app {
    ($f:expr) => {
        crate::event::send_event(
            crate::event::Event::AppState(Box::new(move |app: &mut crate::app::App| {
                $f(app);
            }))
        )
    };
}