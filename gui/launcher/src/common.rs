#[derive(Debug, Clone, Copy)]
pub enum Msg
{
  DrawCover,
  DrawSelector,
  DrawEnv,
  DrawMenu,
  Launch,
  Quit,
} // enum

#[macro_export]
macro_rules! assign_to_arc_mutex
{
  ($arc_mutex:expr, $value:expr) =>
  {
    {
      let mut data = $arc_mutex.lock().unwrap();
      *data = $value;
    }
  };
}

#[macro_export]
macro_rules! call_with_args
{
  ($func:ident, $( $obj:expr ),* ) =>
  {
    $(
      $obj.$func();
    )*
  };
}
