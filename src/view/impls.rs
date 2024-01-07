use crate::*;

impl View for items::Create {
    fn new() -> Self {
        Self {
            aid: -1,
            ..Default::default()
        }
    }

    fn get_parent(&self) -> Res<i32> {
        Ok(self.parent)
    }

    fn get_aid(&self) -> Res<i32> {
        Ok(self.aid)
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

impl View for activity::Activity {
    fn get_tid(&self) -> Res<i32> {
        Ok(self.tid)
    }
}

impl View for items::NewActivityResponse {
    fn get_aid(&self) -> Res<i32> {
        Ok(self.aid)
    }

    fn get_tid(&self) -> Res<i32> {
        Ok(self.tid)
    }
}

impl View for items::View {
    fn get_aid(&self) -> Res<i32> {
        Ok(self.aid)
    }

    fn get_id(&self) -> Res<i32> {
        Ok(self.id)
    }
}
