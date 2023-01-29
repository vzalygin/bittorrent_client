use std::path::Path;

use uuid::Uuid;

use crate::common_types::{data::Torrent, error::AsyncErr};

pub type Id = Uuid;

pub struct WithId<T> {
    pub id: Id,
    pub value: T,
}

pub struct TorrentRepo {
    torrents: Vec<WithId<Torrent>>,
}

impl TorrentRepo {
    pub fn get_torrent_list(&self) -> &Vec<WithId<Torrent>> {
        &self.torrents
    }

    pub fn add_new_torrent(&mut self, torrent: Torrent) {
        let torrent = WithId {
            id: Uuid::new_v4(),
            value: torrent,
        };

        self.torrents.push(torrent)
    }

    /// Возвращает `true`, если значение было изменено.'7kl
    pub fn edit_torrent(&mut self, torrent: WithId<Torrent>) -> bool {
        let old = self
            .torrents
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
        let torrent = self.torrents.iter().enumerate().find(|t| t.1.id == id);

        if let Some(torrent) = torrent {
            let (i, _) = torrent;
            self.torrents.remove(i);
            true
        } else {
            false
        }
    }

    pub async fn save_to(&self, path: &Path) -> Result<(), AsyncErr> {
        unimplemented!()
    }

    pub async fn load_from(path: &Path) -> Result<TorrentRepo, AsyncErr> {
        let file = &tokio::fs::read(path).await?;

        match file[..].try_into() {
            Ok(repo) => Ok(repo),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn from(torrents: Vec<WithId<Torrent>>) -> TorrentRepo {
        TorrentRepo { torrents }
    }
}
