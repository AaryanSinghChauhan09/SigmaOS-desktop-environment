#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopWorkspace {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Default)]
pub struct DesktopEnvironmentCoordinator {
    workspaces: Vec<DesktopWorkspace>,
    active_workspace: u32,
}

impl DesktopEnvironmentCoordinator {
    pub fn new() -> Self {
        Self {
            workspaces: vec![
                DesktopWorkspace { id: 1, name: "Workspace 1".into() },
                DesktopWorkspace { id: 2, name: "Workspace 2".into() },
            ],
            active_workspace: 1,
        }
    }
    pub fn switch_workspace(&mut self, id: u32) {
        if self.workspaces.iter().any(|w| w.id == id) {
            self.active_workspace = id;
        }
    }
    pub fn current(&self) -> u32 { self.active_workspace }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_desktop_environment_coordinator_aaa() {
        // Arrange
        let mut de = DesktopEnvironmentCoordinator::new();
        assert_eq!(de.current(), 1);
        // Act
        de.switch_workspace(2);
        // Assert
        assert_eq!(de.current(), 2);
    }
}
