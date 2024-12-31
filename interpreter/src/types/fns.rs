use crate::Package;
use chalk_rs::Chalk;
use std::collections::HashMap;

use super::{HeapWrapper, Options};

//pub trait DynPackageCallback = FnMut(&Args, &mut Heap, &mut bool);
pub type Args = Vec<String>;
pub type PackageCallback = fn(&Args, HeapWrapper, &String, &mut Options) -> ();

pub type DynMethodRes = Vec<(&'static str, &'static str, PackageCallback)>;
pub type MethodRes = &'static [(&'static str, &'static str, PackageCallback)];

pub enum MethodData<'a> {
  Static(&'a str, PackageCallback),
}

pub struct LanguagePackages<'a> {
  pub inner: HashMap<&'static str, MethodData<'a>>,
  pub docs: HashMap<&'static str, &'static str>,
}

impl<'a> LanguagePackages<'a> {
  pub fn new() -> Self {
    Self {
      inner: HashMap::new(),
      docs: HashMap::new(),
    }
  }

  pub fn import<T: Package>(&mut self, func: T) -> &mut Self {
    let name = String::from_utf8_lossy(func.name());
    let name: &'static mut str = name.to_string().leak::<'static>();
    for (key, doc, val) in func.methods() {
      self.inner.insert(key, MethodData::Static(name, *val));
      self.docs.insert(key, doc);
    }
    for (k, doc, v) in func.dyn_methods() {
      self.inner.insert(k, MethodData::Static(name, v));
      self.docs.insert(k, doc);
    }
    self
  }

  pub fn list(&self, chalk: &mut Chalk) {
    println!(
      "{} {}",
      chalk.reset_weight().blue().string(&"Total Commands:"),
      self.inner.len()
    );
    chalk.reset_weight().green().println(&"Commands:");

    self
      .inner
      .iter()
      .enumerate()
      .for_each(|(no, (syntax, mt))| match mt {
        MethodData::Static(name, _) => {
          chalk.red().print(&format!("{}- ", no + 1));
          chalk.yellow().bold().print(&syntax);
          print!(" from ");
          chalk.reset_weight().blue().println(&name);
        }
      });
  }
}
