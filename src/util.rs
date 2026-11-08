//! AppImage toolbox model — AppImage management with advanced features.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, PartialEq)]
pub enum AppImageType {
    Type1, // Traditional AppImage
    Type2, // SquashFS-based AppImage
    Type3, // OCI-based AppImage
}

#[derive(Debug, Clone)]
pub struct AppImage {
    pub name: String,
    pub path: String,
    pub type_: AppImageType,
    pub version: String,
    pub architecture: String,
    pub size: u64,
    pub checksum: String,
    pub integrated: bool,
    pub desktop_entry: Option<DesktopEntry>,
    pub icon: Option<String>,
    pub dependencies: Vec<String>,
    pub update_info: Option<UpdateInfo>,
    pub created_at: SystemTime,
    pub modified_at: SystemTime,
}

#[derive(Debug, Clone)]
pub struct DesktopEntry {
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
    pub categories: Vec<String>,
    pub mime_types: Vec<String>,
    pub terminal: bool,
    pub actions: Vec<DesktopAction>,
}

#[derive(Debug, Clone)]
pub struct DesktopAction {
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub update_url: String,
    pub check_interval: Duration,
    pub last_check: Option<SystemTime>,
    pub latest_version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Integration {
    pub appimage_name: String,
    pub desktop_file: String,
    pub icon_path: String,
    pub mime_types: Vec<String>,
    pub integrated_at: SystemTime,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub appimages: HashMap<String, AppImage>,
    pub integrations: HashMap<String, Integration>,
    pub update_checks: Vec<UpdateCheck>,
    pub max_appimages: usize,
    pub default_integration_dir: String,
}

#[derive(Debug, Clone)]
pub struct UpdateCheck {
    pub appimage_name: String,
    pub checked_at: SystemTime,
    pub update_available: bool,
    pub latest_version: Option<String>,
    pub error: Option<String>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            appimages: HashMap::new(),
            integrations: HashMap::new(),
            update_checks: vec![],
            max_appimages: 1000,
            default_integration_dir: "/home/user/.local/share/applications".into(),
        }
    }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_appimage(&mut self, appimage: AppImage) -> String {
        let name = appimage.name.clone();
        self.appimages.insert(name.clone(), appimage);
        if self.appimages.len() > self.max_appimages {
            // Remove oldest appimage
            let oldest = self.appimages.values()
                .min_by_key(|a| a.created_at)
                .map(|a| a.name.clone());
            if let Some(oldest_name) = oldest {
                self.appimages.remove(&oldest_name);
            }
        }
        name
    }

    pub fn get_appimage(&self, name: &str) -> Option<&AppImage> {
        self.appimages.get(name)
    }

    pub fn remove_appimage(&mut self, name: &str) -> bool {
        self.appimages.remove(name).is_some()
    }

    pub fn search_appimages(&self, query: &str) -> Vec<&AppImage> {
        let query_lower = query.to_lowercase();
        self.appimages.values()
            .filter(|a| {
                a.name.to_lowercase().contains(&query_lower) ||
                a.version.to_lowercase().contains(&query_lower) ||
                a.architecture.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    pub fn get_integrated_appimages(&self) -> Vec<&AppImage> {
        self.appimages.values()
            .filter(|a| a.integrated)
            .collect()
    }

    pub fn integrate_appimage(&mut self, name: &str) -> bool {
        if let Some(appimage) = self.appimages.get_mut(name) {
            if appimage.desktop_entry.is_some() {
                appimage.integrated = true;
                let integration = Integration {
                    appimage_name: name.into(),
                    desktop_file: format!("{}.desktop", name),
                    icon_path: appimage.icon.clone().unwrap_or_default(),
                    mime_types: appimage.desktop_entry.as_ref()
                        .map(|de| de.mime_types.clone())
                        .unwrap_or_default(),
                    integrated_at: SystemTime::now(),
                };
                self.integrations.insert(name.into(), integration);
                return true;
            }
        }
        false
    }

    pub fn unintegrate_appimage(&mut self, name: &str) -> bool {
        if let Some(appimage) = self.appimages.get_mut(name) {
            appimage.integrated = false;
            self.integrations.remove(name);
            return true;
        }
        false
    }

    pub fn get_integration(&self, name: &str) -> Option<&Integration> {
        self.integrations.get(name)
    }

    pub fn set_desktop_entry(&mut self, name: &str, entry: DesktopEntry) -> bool {
        if let Some(appimage) = self.appimages.get_mut(name) {
            appimage.desktop_entry = Some(entry);
            return true;
        }
        false
    }

    pub fn set_update_info(&mut self, name: &str, info: UpdateInfo) -> bool {
        if let Some(appimage) = self.appimages.get_mut(name) {
            appimage.update_info = Some(info);
            return true;
        }
        false
    }

    pub fn check_for_updates(&mut self, name: &str) -> Option<UpdateCheck> {
        let appimage = self.appimages.get(name)?;
        let check = UpdateCheck {
            appimage_name: name.into(),
            checked_at: SystemTime::now(),
            update_available: false,
            latest_version: None,
            error: None,
        };
        self.update_checks.push(check.clone());
        Some(check)
    }

    pub fn get_update_checks(&self, name: &str) -> Vec<&UpdateCheck> {
        self.update_checks.iter()
            .filter(|c| c.appimage_name == name)
            .collect()
    }

    pub fn verify_checksum(&self, name: &str, checksum: &str) -> bool {
        self.appimages.get(name)
            .map(|a| a.checksum == checksum)
            .unwrap_or(false)
    }

    pub fn get_appimage_type(&self, name: &str) -> Option<&AppImageType> {
        self.appimages.get(name).map(|a| &a.type_)
    }

    pub fn get_dependencies(&self, name: &str) -> Vec<&String> {
        self.appimages.get(name)
            .map(|a| a.dependencies.iter().collect())
            .unwrap_or_default()
    }

    pub fn appimages(&self) -> &HashMap<String, AppImage> { &self.appimages }
    pub fn integrations(&self) -> &HashMap<String, Integration> { &self.integrations }
    pub fn update_checks(&self) -> &[UpdateCheck] { &self.update_checks }
    pub fn max_appimages(&self) -> usize { self.max_appimages }
    pub fn default_integration_dir(&self) -> &str { &self.default_integration_dir }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appimage_basic_operations() {
        let mut m = Model::new();
        assert!(m.appimages.is_empty());
        assert!(m.integrations.is_empty());
    }

    #[test]
    fn appimage_management() {
        let mut m = Model::new();
        let appimage = AppImage {
            name: "test-app".into(),
            path: "/path/to/app.AppImage".into(),
            type_: AppImageType::Type2,
            version: "1.0".into(),
            architecture: "x86_64".into(),
            size: 1024 * 1024,
            checksum: "abc123".into(),
            integrated: false,
            desktop_entry: None,
            icon: None,
            dependencies: vec![],
            update_info: None,
            created_at: SystemTime::now(),
            modified_at: SystemTime::now(),
        };
        let name = m.add_appimage(appimage);
        assert!(m.appimages.contains_key(&name));
        assert!(m.get_appimage(&name).is_some());
    }

    #[test]
    fn appimage_search() {
        let mut m = Model::new();
        let appimage1 = AppImage {
            name: "test-app".into(),
            path: "/path/to/app1.AppImage".into(),
            type_: AppImageType::Type2,
            version: "1.0".into(),
            architecture: "x86_64".into(),
            size: 1024,
            checksum: "abc123".into(),
            integrated: false,
            desktop_entry: None,
            icon: None,
            dependencies: vec![],
            update_info: None,
            created_at: SystemTime::now(),
            modified_at: SystemTime::now(),
        };
        let appimage2 = AppImage {
            name: "other-app".into(),
            path: "/path/to/app2.AppImage".into(),
            type_: AppImageType::Type1,
            version: "2.0".into(),
            architecture: "x86_64".into(),
            size: 2048,
            checksum: "def456".into(),
            integrated: false,
            desktop_entry: None,
            icon: None,
            dependencies: vec![],
            update_info: None,
            created_at: SystemTime::now(),
            modified_at: SystemTime::now(),
        };
        m.add_appimage(appimage1);
        m.add_appimage(appimage2);
        let results = m.search_appimages("test");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "test-app");
    }

    #[test]
    fn appimage_integration() {
        let mut m = Model::new();
        let appimage = AppImage {
            name: "test-app".into(),
            path: "/path/to/app.AppImage".into(),
            type_: AppImageType::Type2,
            version: "1.0".into(),
            architecture: "x86_64".into(),
            size: 1024,
            checksum: "abc123".into(),
            integrated: false,
            desktop_entry: Some(DesktopEntry {
                name: "Test App".into(),
                exec: "test-app".into(),
                icon: Some("test-icon".into()),
                categories: vec!["Utility".into()],
                mime_types: vec!["text/plain".into()],
                terminal: false,
                actions: vec![],
            }),
            icon: Some("test-icon".into()),
            dependencies: vec![],
            update_info: None,
            created_at: SystemTime::now(),
            modified_at: SystemTime::now(),
        };
        let name = m.add_appimage(appimage);
        assert!(m.integrate_appimage(&name));
        assert!(m.get_appimage(&name).unwrap().integrated);
        assert!(m.get_integration(&name).is_some());
        assert!(m.unintegrate_appimage(&name));
        assert!(!m.get_appimage(&name).unwrap().integrated);
        assert!(m.get_integration(&name).is_none());
    }

    #[test]
    fn appimage_update_check() {
        let mut m = Model::new();
        let appimage = AppImage {
            name: "test-app".into(),
            path: "/path/to/app.AppImage".into(),
            type_: AppImageType::Type2,
            version: "1.0".into(),
            architecture: "x86_64".into(),
            size: 1024,
            checksum: "abc123".into(),
            integrated: false,
            desktop_entry: None,
            icon: None,
            dependencies: vec![],
            update_info: Some(UpdateInfo {
                update_url: "https://example.com/updates".into(),
                check_interval: Duration::from_secs(86400),
                last_check: None,
                latest_version: None,
            }),
            created_at: SystemTime::now(),
            modified_at: SystemTime::now(),
        };
        let name = m.add_appimage(appimage);
        let check = m.check_for_updates(&name);
        assert!(check.is_some());
        assert_eq!(m.get_update_checks(&name).len(), 1);
    }

    #[test]
    fn appimage_checksum_verification() {
        let mut m = Model::new();
        let appimage = AppImage {
            name: "test-app".into(),
            path: "/path/to/app.AppImage".into(),
            type_: AppImageType::Type2,
            version: "1.0".into(),
            architecture: "x86_64".into(),
            size: 1024,
            checksum: "abc123".into(),
            integrated: false,
            desktop_entry: None,
            icon: None,
            dependencies: vec![],
            update_info: None,
            created_at: SystemTime::now(),
            modified_at: SystemTime::now(),
        };
        let name = m.add_appimage(appimage);
        assert!(m.verify_checksum(&name, "abc123"));
        assert!(!m.verify_checksum(&name, "wrong"));
    }
}
