use crate::find::Find;
use crate::structure::HasParent;
use uuid::Uuid;

pub trait Create {
    type Options;
    fn create(options: Self::Options) -> Self;
}

pub trait AddItem<Item> {
    fn add_item(&mut self, item: Item);
}

pub trait Add<Item>
where
    Self: AddItem<Item>,
    Item: Create,
{
    fn add(&mut self, options: Item::Options) {
        let item = Item::create(options);
        self.add_item(item);
    }
}

pub trait AddChild<Item>
where
    Self: Find<Item::Parent>,
    Item: HasParent + Create,
    Item::Parent: Add<Item>,
{
    fn add_child(&mut self, uuid: &Uuid, options: <Item as Create>::Options) {
        if let Some(parent) = self.find_mut(uuid) {
            parent.add(options);
        }
    }
}

pub trait AddParent
where
    Self: Sized + HasParent,
    Self::Parent: Create + AddItem<Self>,
{
    fn add_parent(self, options: <Self::Parent as Create>::Options) -> Self::Parent {
        let mut parent = Self::Parent::create(options);
        parent.add_item(self);
        parent
    }
}
