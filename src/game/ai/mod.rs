pub trait World<T, P> {
    fn has_at(entity: T, p: P) -> bool;
}

pub trait Sensor<T, P> {
    type W = World<T, P>;

    fn is_matching(world: W) -> bool;
}

pub trait Action {
    fn exec();
}
