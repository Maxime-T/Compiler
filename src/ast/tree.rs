use std::{fs::File, path::Path};
use super::grammar::Body;

pub struct Tree {
  body : Body,
}

impl Tree {
  pub fn from(file : &Path) {
    let source = File::open(file);
     
  }
}

