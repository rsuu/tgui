use crate::*;

impl View for items::View {
    fn id(&self) -> Res<i32> {
        Ok(self.id)
    }

    fn aid(&self) -> Res<i32> {
        Ok(self.aid)
    }

    fn act(&self) -> &Activity {
        unreachable!()
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
    fn parent(&self) -> Res<i32> {
        Ok(self.parent)
    }

    fn visible(&self) -> Res<i32> {
        Ok(self.v)
    }

    fn act(&self) -> &Activity {
        unreachable!()
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

    fn set_v(mut self, v: items::Visibility) -> Self {
        self.v = v as i32;

        self
    }
}
