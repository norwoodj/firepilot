use crate::builder::{Builder, BuilderError};
use firepilot_models::models::MachineConfiguration;

#[derive(Debug)]
pub struct MachineConfigurationBuilder {
    pub vcpu_count: Option<i32>,
    pub mem_mib: Option<i32>,
}

impl Default for MachineConfigurationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MachineConfigurationBuilder {
    pub fn new() -> Self {
        Self {
            vcpu_count: None,
            mem_mib: None,
        }
    }

    pub fn with_vcpu_count(mut self, vcpu_count: i32) -> MachineConfigurationBuilder {
        self.vcpu_count = Some(vcpu_count);
        self
    }

    pub fn with_mem_mib(mut self, mem_mib: i32) -> MachineConfigurationBuilder {
        self.mem_mib = Some(mem_mib);
        self
    }
}

impl Builder<MachineConfiguration> for MachineConfigurationBuilder {
    fn try_build(self) -> Result<MachineConfiguration, BuilderError> {
        Ok(MachineConfiguration {
            cpu_template: None,
            smt: None,
            mem_size_mib: self.mem_mib.unwrap_or_default(),
            track_dirty_pages: None,
            vcpu_count: self.vcpu_count.unwrap_or_default(),
        })
    }
}
