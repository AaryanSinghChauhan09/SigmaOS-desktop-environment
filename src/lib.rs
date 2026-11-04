//! SigmaOS Desktop Environment
//! Main desktop environment with window management, panels, docks

extern crate alloc;

pub mod desktop;
pub mod compositor;
pub mod shell;
pub mod cinnamon;

pub use desktop::DesktopEnvironment;

pub mod klib {
    pub use std::collections::*;
    pub mod btreemap {
        pub use std::collections::BTreeMap;
    }
}

pub mod ai {
    pub mod agent_runtime {
        pub type AgentId = u64;
        pub struct AgentKernelBridge;
    }
}

pub mod accessibility {
    #[derive(Debug, Clone)]
    pub struct AccessibilityFeature;
    #[derive(Debug, Clone)]
    pub struct AccessibilityFramework;
    #[derive(Debug, Clone)]
    pub struct AccessibilitySetting;
}

pub mod compatibility {
    #[derive(Debug, Clone)]
    pub struct ApplicationBinary;
    #[derive(Debug, Clone)]
    pub struct BinaryFormat;
    #[derive(Debug, Clone)]
    pub struct CompatibilityManager;
    #[derive(Debug, Clone)]
    pub struct TargetPlatform;
}

pub mod customization {
    #[derive(Debug, Clone)]
    pub struct CustomizationEngine;
}

pub mod dashboard {
    #[derive(Debug, Clone)]
    pub struct SystemMonitor;
}

pub mod package {
    #[derive(Debug, Clone)]
    pub struct UnifiedPackage;
    #[derive(Debug, Clone)]
    pub struct UniversalPackageManager;
}

pub mod resilience {
    #[derive(Debug, Clone)]
    pub struct SelfHealingModule;
}

pub mod virtualization {
    #[derive(Debug, Clone)]
    pub struct Container;
    #[derive(Debug, Clone)]
    pub struct VirtualMachine;
    #[derive(Debug, Clone)]
    pub struct VirtualizationOrchestrator;
    #[derive(Debug, Clone)]
    pub struct VirtualizationTech;
}
