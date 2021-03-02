use crate::error::Result;
use crate::error::WrapErrorExt;

pub(crate) fn register_handler<T>(handler: T) -> Result<()>
where
    T: Fn() + 'static + Send,
{
    ctrlc::set_handler(handler).wrap_error("termination", "failed to set termination handler")
}
