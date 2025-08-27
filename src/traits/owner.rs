use core::fmt::Debug;

pub trait Owner {
    type OwnerId: Copy + Eq + Debug;

    fn owner_id(&self) -> Self::OwnerId;
}
