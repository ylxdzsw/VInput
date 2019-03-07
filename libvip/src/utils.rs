pub trait Ignore {
    fn ignore(&self) {}
}

impl<T> Ignore for T {}

pub trait Apply {
    fn apply<F: FnOnce(&Self)>(&self, f: F) -> &Self {
        f(self); self
    }

    fn apply_mut<F: FnOnce(&mut Self)>(&mut self, f: F) -> &mut Self {
        f(self); self
    }
}

impl<T> Apply for T {}

pub trait ApplyOwned: Sized {
    fn apply_owned<F: FnOnce(&mut Self)>(mut self, f: F) -> Self {
        f(&mut self); self
    }
}

impl<T: Sized> ApplyOwned for T {}

