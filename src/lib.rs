#[cfg(test)]
extern crate regex;

extern crate curl;

use curl::http;
use std::str;

#[deriving(Show)]
pub enum ApiError {
  None,
  RequestFailure,
  InvalidResponse
}

fn get (url: &str) -> Result<String, ApiError> {
  let res = match http::handle().get(url).exec() {
    Ok(res) => res,
    Err(_) => return Err(ApiError::RequestFailure)
  };

  let body = match str::from_utf8(res.get_body()) {
    Ok(body) => body,
    Err(_) => return Err(ApiError::InvalidResponse)
  };

  Ok(body.to_string())
}

pub fn versions (dist: String) -> Result<Vec<String>, ApiError> {
  let body = try!(get(format!("https://semver.io/{}/versions", dist).as_slice()));

  let versions = body.lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

  Ok(versions)
}

pub fn stable (dist: String) -> Result<String, ApiError> {
  let url = format!("https://semver.io/{}/stable", dist);
  Ok(try!(get(url.as_slice())))
}

pub fn unstable (dist: String) -> Result<String, ApiError> {
  let url = format!("https://semver.io/{}/unstable", dist);
  Ok(try!(get(url.as_slice())))
}

pub fn resolve (dist: String, version: String) -> Result<String, ApiError> {
  let url = format!("https://semver.io/{}/resolve/{}", dist, version);
  Ok(try!(get(url.as_slice())))
}

#[cfg(test)]
mod tests {
  use regex::{Regex, NoExpand};

  fn is_semver (input: String) -> bool {
    let r = Regex::new(r"^[v]?[0-9]*\.[0-9]*\.[0-9]*").unwrap();
    match r.captures(input.as_slice()) {
      Some(c) => true,
      None => false,
    }
  }

  #[test]
  fn test_versions () {
    let list = super::versions("iojs".to_string()).ok().unwrap();
    for version in list.iter() {
      assert!(is_semver(version.to_string()));
    }
  }

  #[test]
  fn test_stable () {
    assert!(is_semver(super::stable("iojs".to_string()).ok().unwrap()));
  }

  #[test]
  fn test_unstable () {
    assert!(is_semver(super::unstable("iojs".to_string()).ok().unwrap()));
  }

  #[test]
  fn test_resolve () {
    assert!(is_semver(super::resolve("iojs".to_string(), "1.x".to_string()).ok().unwrap()));
  }
}
