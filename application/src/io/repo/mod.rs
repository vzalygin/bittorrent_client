use std::path::Path;

use crate::common_types::{data::Torrent, error::AsyncErr};

pub type Id = u32;

pub struct WithId<T> {
    pub id: Id,
    pub value: T,
}

pub struct TorrentRepo {
    torrents: Vec<WithId<Torrent>>,
    next_id: Id,
}

impl TorrentRepo {
    fn gen_id(&mut self) -> Id {
        self.next_id += 1;
        self.next_id - 1
    }

    pub fn get_torrent_list(&self) -> &Vec<WithId<Torrent>> {
        &self.torrents
    }

    pub fn add_new_torrent(&mut self, torrent: Torrent) {
        let torrent = WithId {
            id: self.gen_id(),
            value: torrent,
        };
        self.torrents.push(torrent)
    }

    /// Возвращает `true`, если значение было изменено.
    pub fn edit_torrent(&mut self, torrent: WithId<Torrent>) -> bool {
        let old = self.torrents
            .iter()
            .enumerate()
            .find(|t| t.1.id == torrent.id);

        if let Some(old) = old {
            let (i, _) = old;
            self.torrents[i] = torrent;
            true 
        } else {
            false
        }
    }

    /// Возвращает 'true', если значение было удалено.
    pub fn remove_torrent_by_id(&mut self, id: Id) -> bool {
        unimplemented!()
    }

    pub async fn save_to(&self, path: &Path) -> Result<(), AsyncErr> {
        unimplemented!()
    }

    pub async fn load_from(path: &Path) -> Result<TorrentRepo, AsyncErr> {
        unimplemented!()
    }
}
