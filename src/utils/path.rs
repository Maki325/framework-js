use std::{
  env,
  error::Error,
  path::{Component, PathBuf},
};

pub fn normalize_path(path: &PathBuf) -> PathBuf {
  let mut components = path.components().peekable();
  let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
    components.next();
    PathBuf::from(c.as_os_str())
  } else {
    PathBuf::new()
  };

  for component in components {
    match component {
      Component::Prefix(..) => unreachable!(),
      Component::RootDir => {
        ret.push(component.as_os_str());
      }
      Component::CurDir => {}
      Component::ParentDir => {
        ret.pop();
      }
      Component::Normal(c) => {
        ret.push(c);
      }
    }
  }
  ret
}

pub fn make_abs_path(path: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
  let path = if path.is_absolute() {
    path
  } else {
    env::current_dir()?.join(path)
  };

  return Ok(normalize_path(&path));
}
