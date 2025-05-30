use std::any::Any;
use std::borrow::Cow;

pub trait PanicReason<'a, T> {
    fn panic_reason(&self) -> Option<Cow<'a, str>>;
}

impl<'a, T> PanicReason<'a, T> for Result<T, Box<dyn Any + Send + 'a>> {
    fn panic_reason(&self) -> Option<Cow<'a, str>> {
        match self {
            Ok(_) => None,
            Err(err) => match (err.downcast_ref::<&'a str>(), err.downcast_ref::<&'a String>()) {
                (Some(s), _) => Some(Cow::from(*s)),
                (_, Some(s)) => Some(Cow::from(*s)),
                _ => Some(Cow::from("Unknown error from joined thread"))
            }
        }
    }
}
