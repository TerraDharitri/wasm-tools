use std::fmt;

use semver::Version;

use crate::{ident::Ident, Interface, Render, RenderOpts, World};

/// A WIT package.
///
/// A package is a collection of interfaces and worlds. Packages additionally
/// have a unique identifier that affects generated components and uniquely
/// identifiers this particular package.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub struct Package {
    /// A unique name corresponding to this package.
    name: PackageName,

    /// World items
    items: Vec<PackageItem>,
}

impl Package {
    /// Create a new instance of `Package`.
    pub fn new(name: PackageName) -> Self {
        Self {
            name,
            items: vec![],
        }
    }

    pub fn name(&self) -> &PackageName {
        &self.name
    }

    pub fn set_name(&mut self, name: impl Into<PackageName>) {
        self.name = name.into();
    }

    /// Add an `Interface` to the package
    pub fn interface(&mut self, interface: Interface) {
        self.items.push(PackageItem::Interface(interface))
    }

    /// Add a `World` to the package
    pub fn world(&mut self, world: World) {
        self.items.push(PackageItem::World(world))
    }

    pub fn item(&mut self, item: impl Into<PackageItem>) {
        self.items.push(item.into());
    }

    pub fn items(&self) -> &[PackageItem] {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut Vec<PackageItem> {
        &mut self.items
    }
}

impl Render for Package {
    fn render(&self, f: &mut fmt::Formatter<'_>, opts: &RenderOpts) -> fmt::Result {
        write!(f, "{}package {};\n", opts.spaces(), self.name)?;
        for item in &self.items {
            write!(f, "\n")?;
            match item {
                PackageItem::Interface(interface) => {
                    if let Some(docs) = &interface.docs {
                        docs.render(f, opts)?;
                    }
                    write!(f, "{}interface {} {{", opts.spaces(), interface.name)?;
                    if !interface.uses.is_empty() || !interface.items.is_empty() {
                        write!(f, "\n")?;
                        interface.uses.render(f, &opts.indent())?;
                        interface.items.render(f, &opts.indent())?;
                        write!(f, "{}}}\n", opts.spaces())?;
                    } else {
                        write!(f, "}}\n")?;
                    }
                }
                PackageItem::World(world) => {
                    world.render(f, opts)?;
                }
            }
        }
        Ok(())
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render(f, &RenderOpts::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum PackageItem {
    Interface(Interface),
    World(World),
}

/// A structure used to keep track of the name of a package, containing optional
/// information such as a namespace and version information.
///
/// This is directly encoded as an "ID" in the binary component representation
/// with an interfaced tacked on as well.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub struct PackageName {
    /// A namespace such as `wasi` in `wasi:foo/bar`
    namespace: String,
    /// The kebab-name of this package, which is always specified.
    name: Ident,
    /// Optional major/minor version information.
    version: Option<Version>,
}

impl PackageName {
    /// Create a new instance of `PackageName`
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<Ident>,
        version: Option<Version>,
    ) -> Self {
        Self {
            namespace: namespace.into(),
            name: name.into(),
            version,
        }
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn set_namespace(&mut self, namespace: impl Into<String>) {
        self.namespace = namespace.into();
    }

    pub fn name(&self) -> &Ident {
        &self.name
    }

    pub fn set_name(&mut self, name: impl Into<Ident>) {
        self.name = name.into()
    }

    pub fn version(&self) -> Option<&Version> {
        self.version.as_ref()
    }

    pub fn set_version(&mut self, version: Option<impl Into<Version>>) {
        self.version = version.map(|v| v.into())
    }
}

impl From<PackageName> for String {
    fn from(name: PackageName) -> String {
        name.to_string()
    }
}

impl fmt::Display for PackageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.name)?;
        if let Some(version) = &self.version {
            write!(f, "@{version}")?;
        }
        Ok(())
    }
}
