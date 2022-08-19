#[macro_export]
macro_rules! switch_app_state {
    ($e:expr) => {
        (|mut app_state: ResMut<State<AppState>>| {
            let _ = app_state.set($e);
        })
    };
}

#[macro_export]
macro_rules! impl_new{
  ($to:ty,$($v:ident: $t:ty),*)  => {
      impl $to {
          pub fn new($($v: $t),*) -> $to
          {
              Self {
                  $($v),*
              }
          }
      }
  };
}

#[macro_export]
macro_rules! impl_default {
    ($to:ty) => {
        impl Default for $to {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}
