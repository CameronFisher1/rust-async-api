use crate::domain::user::User;
use crate::repository::UserRepository;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use uuid::Uuid;

pub struct InMemoryUserRepository {
    users: Mutex<HashMap<Uuid, User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
        }
    }

    fn lock_users(&self) -> Result<MutexGuard<'_, HashMap<Uuid, User>>, ()> {
        self.users.lock().map_err(|_| ())
    }
}

impl UserRepository for InMemoryUserRepository {
    fn create(&self, user: User) -> Result<User, ()> {
        let mut users = self.lock_users()?;
        users.insert(user.id, user.clone());
        Ok(user)
    }

    fn get_all(&self) -> Result<Vec<User>, ()> {
        let users = self.lock_users()?;
        Ok(users.values().cloned().collect())
    }

    fn get_by_id(&self, id: Uuid) -> Result<Option<User>, ()> {
        let users = self.lock_users()?;
        Ok(users.get(&id).cloned())
    }

    fn update(&self, user: User) -> Result<Option<User>, ()> {
        let mut users = self.lock_users()?;
        if users.contains_key(&user.id) {
            users.insert(user.id.clone(), user.clone());
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    fn delete(&self, id: Uuid) -> Result<bool, ()> {
        let mut users = self.lock_users()?;
        Ok(users.remove(&id).is_some())
    }
}
