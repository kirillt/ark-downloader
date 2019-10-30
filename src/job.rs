use evmap::shallow_copy::ShallowCopy;

pub struct Job {
    pub id: u64,
//    pub future: Box<dyn Future<Item = (), Error = ()> + Send + Sync>,
}

impl ShallowCopy for Job {
    unsafe fn shallow_copy(&mut self) -> Self {
        Job {
            id: self.id.shallow_copy()
        }
    }
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Job {}