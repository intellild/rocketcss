use std::{marker::PhantomData, rc::Rc};

type InvariantLifetime<'ghost> = PhantomData<fn(&'ghost mut &'ghost ()) -> &'ghost mut &'ghost ()>;

/// A fresh invariant brand used by visitor APIs.
pub struct GhostToken<'ghost> {
    _brand: InvariantLifetime<'ghost>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

impl<'ghost> GhostToken<'ghost> {
    #[inline]
    fn branded() -> Self {
        Self {
            _brand: PhantomData,
            _not_send_or_sync: PhantomData,
        }
    }
}

impl GhostToken<'_> {
    #[inline]
    pub fn scope<R>(f: impl for<'ghost> FnOnce(GhostToken<'ghost>) -> R) -> R {
        f(GhostToken::branded())
    }
}
