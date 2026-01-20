use tokio::sync::Mutex;

#[derive(Debug, Default)]
pub struct UserFavoriteStateInner {
    pub is_loading: bool,
    pub fav_ids: Vec<String>,
}

impl UserFavoriteStateInner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_saved_favorites(&mut self, fav_ids: Vec<String>) {
        self.fav_ids = fav_ids;
    }

    pub fn add_favorite(&mut self, id: &str) {
        self.fav_ids.push(id.to_string());
    }

    pub fn is_favorite(&self, id: &str) -> bool {
        self.fav_ids.contains(&id.to_string())
    }

    pub fn remove_favorite(&mut self, id: &str) {
        self.fav_ids.retain(|x| x != id);
    }

    pub async fn get_favorite_ids(&self) -> Vec<String> {
        self.fav_ids.clone()
    }
}

pub type UserFavoriteState = Mutex<UserFavoriteStateInner>;
