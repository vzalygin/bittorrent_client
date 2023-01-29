use crate::common_types::data::Torrent;

pub type Id = u32;

pub struct WithId<T> {
    pub id: u32,
    pub value: T,
}

pub struct TorrentsRepo {
    torrents: Vec<WithId<Torrent>>,
}

impl TorrentsRepo {
    fn add_new_torrent(&self, torrent: Torrent) -> bool {
        unimplemented!()
    }

    fn remove_torrent_by_id(&self, id: Id) -> bool {
        unimplemented!()
    }
}
