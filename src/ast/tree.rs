use std::{fs::File, path::Path};
use std::io;

use super::grammar::{Body, LineEnum};

pub struct Tree {
  body : Body,
}

impl Tree {
  pub fn new() -> Tree {
    Tree {
      body : Body::new(),
    }
  }

  pub fn from(file : &Path) {
    let source = File::open(file);
     
  }
}

