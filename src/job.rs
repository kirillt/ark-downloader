use evmap::shallow_copy::ShallowCopy;

use futures::future::Future;
use std::process::{Command, Child};
use std::sync::Arc;

pub struct Job {
    pub id: u64,
    state: JobState,
}

enum JobState {
    InProgress(Arc<Child>),
    Finished
}

impl Job {
    pub fn start(id: u64, resource: String) -> Self {
        let handle = Command::new("youtube-dl")
            .arg("--force-ipv4")
            .arg(resource)
            .spawn()
            .expect("Failed to start downloading");

        Job {
            id, state: JobState::InProgress(Arc::new(handle))
        }
    }

    pub fn refresh(&mut self) {
        match &mut self.state {
            JobState::InProgress(handle) =>
                {},
                //if let Some(exit_code) = handle.try_wait().unwrap() {
                //    self.state = JobState::Finished;
                //},
            _ => {}
        }
    }

    pub fn status(&self) -> String {
        match &self.state {
            Finished => "downloaded".to_owned(),
            _ => "scheduled".to_owned()
        }
    }
}

impl ShallowCopy for JobState {
    unsafe fn shallow_copy(&mut self) -> Self {
        match self {
            JobState::InProgress(future) => JobState::InProgress(future.shallow_copy()),
            JobState::Finished => JobState::Finished
        }
    }
}

impl ShallowCopy for Job {
    unsafe fn shallow_copy(&mut self) -> Self {
        Job {
            id: self.id.shallow_copy(),
            state: self.state.shallow_copy()
        }
    }
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Job {}
