use uuid::Uuid;

use crate::domain::service::util::uuid_gen::UUIDGen;

pub struct UUIDGenIMpl;

impl UUIDGen for UUIDGenIMpl {
    fn gen(&self) -> String {
        Uuid::new_v4().to_string()
    }
}
