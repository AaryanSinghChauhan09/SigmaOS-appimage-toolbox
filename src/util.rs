//! AppImage toolbox model — AppImage management.

#[derive(Debug, Clone)]
pub struct AppImage {
    pub name: String,
    pub path: String,
    pub integrated: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub appimages: Vec<AppImage>,
}

impl Default for Model {
    fn default() -> Self { Self { appimages: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, name: &str, path: &str) -> usize {
        let id = self.appimages.len();
        self.appimages.push(AppImage { name: name.into(), path: path.into(), integrated: false });
        id
    }

    pub fn integrate(&mut self, name: &str) -> bool {
        if let Some(a) = self.appimages.iter_mut().find(|a| a.name == name) {
            a.integrated = true;
            return true;
        }
        false
    }

    pub fn remove(&mut self, name: &str) -> bool {
        if let Some(i) = self.appimages.iter().position(|a| a.name == name) {
            self.appimages.remove(i);
            return true;
        }
        false
    }

    pub fn appimages(&self) -> &[AppImage] { &self.appimages }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn appimage_integrate() {
        let mut m = Model::new();
        m.add("app", "/path/app.AppImage");
        assert!(m.integrate("app"));
        assert!(m.remove("app"));
    }
}
