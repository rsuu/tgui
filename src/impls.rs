use crate::*;

impl View for items::View {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self { aid: -1, id: -1 }
    }
}

impl ViewSet for items::View {
    fn set_aid(mut self, aid: i32) -> Self {
        self.aid = aid;

        self
    }

    fn set_id(mut self, id: i32) -> Self {
        self.id = id;

        self
    }
}

impl View for items::Create {
    fn new() -> Self {
        Self {
            aid: -1,
            parent: -1,
            v: 100,
        }
    }

    fn get_parent(&self) -> Res<i32> {
        Ok(self.parent)
    }

    fn get_v(&self) -> Res<i32> {
        Ok(self.v)
    }
}

impl ViewSet for items::Create {
    fn set_parent(mut self, parent: i32) -> Self {
        self.parent = parent;

        self
    }

    fn set_aid(mut self, aid: i32) -> Self {
        self.aid = aid;

        self
    }

    fn set_v(mut self, v: i32) -> Self {
        self.v = v;

        self
    }
}
