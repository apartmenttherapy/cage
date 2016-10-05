//! Overrides modify a pod for use in a specific environment.
//!
//! This module has a plural name to avoid clashing with a keyword.

use project::Project;

/// An `Override` is a collection of extensions to a project's basic pods.
/// Overrides are typically used to represent deployment environments: test,
/// development and production.
///
/// (Right now, this is deliberately a very thin wrapper around the `name`
/// field, suitable for use as key in a `BTreeMap`.  If you add more
/// fields, you'll probably need to remove `PartialEq`, `Eq`, `PartialOrd`,
/// `Ord` from the `derive` list, and either implement them manually or
/// redesign the code that uses overrides as hash table keys.)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Override {
    /// The name of this environment.
    name: String,
}

impl Override {
    /// Create a new override with the specified name.
    pub fn new<S>(name: S) -> Override
        where S: Into<String>
    {
        Override { name: name.into() }
    }

    /// Get the name of this override.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check to see if this override should be included in some operation,
    /// given an optional `enable_in_overrides` overrides list.  If no list
    /// is supplied, we'll act as those we were passed a default list
    /// including all overrides except `test`.
    ///
    /// We have a weird calling convention because our typical callers are
    /// invoking us using a member field of a `Config` struct that they
    /// own.
    ///
    /// ```
    /// let ovr = cage::Override::new("development");
    /// assert!(ovr.is_enabled_by(&None));
    /// assert!(ovr.is_enabled_by(&Some(vec!["development".to_owned()])));
    /// assert!(!ovr.is_enabled_by(&Some(vec!["production".to_owned()])));
    ///
    /// let test_ovr = cage::Override::new("test");
    /// assert!(!test_ovr.is_enabled_by(&None));
    /// ```
    pub fn is_enabled_by(&self, enable_in_overrides: &Option<Vec<String>>) -> bool {
        if let Some(ref enable_in) = *enable_in_overrides {
            // If a list is supplied, we need to appear in it.
            enable_in.contains(&self.name().to_owned())
        } else if self.name() == "test" {
            // `test` is excluded by default.
            false
        } else {
            // All other overrides are included by default.
            true
        }
    }

    /// Get a value for `docker-compose`'s `-p` argument for a given project.
    pub fn compose_project_name(&self, project: &Project) -> String {
        if self.name == "test" {
            format!("{}test", project.name())
        } else {
            project.name().to_owned()
        }
    }
}
