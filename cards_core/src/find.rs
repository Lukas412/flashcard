use crate::structure::HasParent;
use uuid::Uuid;

pub trait Find<Item> {
    fn find(&self, uuid: &Uuid) -> Option<&Item>;
    fn find_mut(&mut self, uuid: &Uuid) -> Option<&mut Item>;
}

pub trait FindParent<Item>
where
    Item: HasParent,
{
    fn find_parent(&self, uuid: &Uuid) -> Option<&Item::Parent>;
    fn find_parent_mut(&mut self, uuid: &Uuid) -> Option<&mut Item::Parent>;
}
