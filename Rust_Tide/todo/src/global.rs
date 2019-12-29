use crate::{
    inmemory::MessageStorage,
    sfs::StaticFileHandler
};


pub struct GlobalState {
    message_storage: MessageStorage,
    static_file_server: StaticFileHandler
}


impl GlobalState {
    pub async fn new(root_path: &str) -> std::io::Result<Self> {
        Ok(Self {
            message_storage: Default::default(),
            static_file_server: StaticFileHandler::new(root_path).await?
        })
    }

    pub fn get_message_storage(&self) -> MessageStorage {
        self.message_storage.clone()
    }

    pub fn get_static_file_server(&self) -> StaticFileHandler {
        self.static_file_server.clone()
    }
}
