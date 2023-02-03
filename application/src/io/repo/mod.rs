use std::{fmt::Debug, path::Path};

use uuid::Uuid;

use crate::{
    common_types::{data::Torrent, error::AsyncErr},
    io::{
        consts::*,
        deserialization::deserialize_torrent_repo,
        serialization::{BencodeDictBuilder, SerializeTo},
    },
};

pub type Id = Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct WithId<T>
where
    T: Clone + PartialEq + Debug,
{
    pub id: Id,
    pub value: T,
}

impl SerializeTo<Vec<u8>> for Id {
    fn serialize(&self) -> Vec<u8> {
        self.as_bytes().to_vec().serialize()
    }
}

impl<T> SerializeTo<Vec<u8>> for WithId<T>
where
    T: SerializeTo<Vec<u8>> + Clone + PartialEq + Debug,
{
    fn serialize(&self) -> Vec<u8> {
        let e = self.clone();
        BencodeDictBuilder::new()
            .required(VALUE, e.value)
            .required(ID, e.id)
            .fin()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TorrentRepo {
    pub torrents: Vec<WithId<Torrent>>,
}

impl SerializeTo<Vec<u8>> for TorrentRepo {
    fn serialize(&self) -> Vec<u8> {
        BencodeDictBuilder::new()
            .required(TORRENTS, self.get_torrent_list().clone())
            .fin()
    }
}

impl TorrentRepo {
    pub fn empty() -> TorrentRepo {
        TorrentRepo { torrents: vec![] }
    }

    pub async fn load_from(path: &Path) -> Result<TorrentRepo, AsyncErr> {
        let file = &tokio::fs::read(path).await?;

        match deserialize_torrent_repo(&file[..]) {
            Ok(repo) => Ok(repo),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn save_to(&self, path: &Path) -> Result<(), AsyncErr> {
        if let Err(e) = tokio::fs::write(path, self.serialize()).await {
            Err(Box::new(e))
        } else {
            Ok(())
        }
    }

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
}
