use evmap::shallow_copy::ShallowCopy;

use std::process::{Command, Child};
use std::sync::Arc;

pub struct Job {
    pub id: u64,
    process: Arc<Child>,
}

static OUTPUT_TEMPLATE_NAME: &str = "%(uploader)s_%(channel_id)s_-_%(title)s_%(id)s";
static OUTPUT_TEMPLATE_EXT:  &str = "%(ext)s";

impl Job {
    pub fn start(id: u64, resource: String, suffix: Option<&str>) -> Self {
        let output_template: &str = &format!("-o {}_{}.{}",
            OUTPUT_TEMPLATE_NAME,
            suffix.unwrap_or(""),
            OUTPUT_TEMPLATE_EXT);

        let handle = Command::new("youtube-dl")
            .arg("--restrict-filenames")
            .arg("--force-ipv4")
            .arg(output_template)
            .arg(resource)
            .spawn()
            .expect("Failed to start downloading");

        Job {
            id, process: Arc::new(handle)
        }
    }
}

impl ShallowCopy for Job {
    unsafe fn shallow_copy(&mut self) -> Self {
        Job {
            id: self.id.shallow_copy(),
            process: self.process.shallow_copy()
        }
    }
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Job {}
