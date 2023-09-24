use crate::find::FindParent;
use crate::structure::HasParent;
use uuid::Uuid;

pub trait Remove<Item> {
    fn remove(&mut self, uuid: &Uuid) -> Option<Item>;
}

pub trait RemoveChild<Item>
where
    Self: FindParent<Item>,
    Item: HasParent,
    Item::Parent: Remove<Item>,
{
    fn remove_child(&mut self, uuid: &Uuid) -> Option<Item> {
        self.find_parent_mut(uuid)
            .and_then(|parent| parent.remove(uuid))
    }
}
