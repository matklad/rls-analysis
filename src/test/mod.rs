// Copyright 2016 The RLS Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {AnalysisHost, AnalysisLoader};
use raw::DefKind;

use std::path::{Path, PathBuf};

#[derive(Clone, new)]
struct TestAnalysisLoader {
    path: PathBuf,
}

impl AnalysisLoader for TestAnalysisLoader {
    fn needs_hard_reload(&self, _path_prefix: &Path) -> bool {
        true
    }

    fn fresh_host(&self) -> AnalysisHost<Self> {
        AnalysisHost::new_with_loader(self.clone())
    }

    fn set_path_prefix(&self, _path_prefix: &Path) {
    }

    fn abs_path_prefix(&self) -> Option<PathBuf> {
        panic!();
    }

    fn iter_paths<F, T>(&self, f: F) -> Vec<T>
        where F: Fn(&Path) -> Vec<T>
    {
        let paths = &[&self.path];
        paths.iter().flat_map(|p| f(p).into_iter()).collect()
    }
}


#[test]
fn smoke() {
    // Read in test data and lower it, check we don't crash.
    let host = AnalysisHost::new_with_loader(TestAnalysisLoader::new(Path::new("test_data/rls-analysis").to_owned()));
    host.reload(Path::new("test_data/rls-analysis"), false).unwrap();
}

#[test]
fn test_hello() {
    // Simple program, a somewhat thorough test that we have all the defs and refs we expect.
    let host = AnalysisHost::new_with_loader(TestAnalysisLoader::new(Path::new("test_data/hello/save-analysis").to_owned()));
    host.reload(Path::new("test_data/hello"), false).unwrap();

    let ids = host.search_for_id("print_hello").unwrap();
    assert_eq!(ids.len(), 1);
    let id = ids[0];
    let def = host.get_def(id).unwrap();
    assert_eq!(def.name, "print_hello");
    assert_eq!(def.kind, DefKind::Function);
    let refs = host.find_all_refs_by_id(id).unwrap();
    assert_eq!(refs.len(), 2);
    assert_eq!(refs[0].file_name, Path::new("test_data/hello/src/main.rs"));
    assert_eq!(refs[0].line_start, 0);
    assert_eq!(refs[1].file_name, Path::new("test_data/hello/src/main.rs"));
    assert_eq!(refs[1].line_start, 6);
    let refs = host.search("print_hello").unwrap();
    assert_eq!(refs.len(), 2);
    assert_eq!(refs[0].file_name, Path::new("test_data/hello/src/main.rs"));
    assert_eq!(refs[0].line_start, 0);
    assert_eq!(refs[1].file_name, Path::new("test_data/hello/src/main.rs"));
    assert_eq!(refs[1].line_start, 6);

    let ids = host.search_for_id("main").unwrap();
    assert_eq!(ids.len(), 1);
    let id = ids[0];
    let def = host.get_def(id).unwrap();
    assert_eq!(def.name, "main");
    assert_eq!(def.kind, DefKind::Function);
    let refs = host.find_all_refs_by_id(id).unwrap();
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].file_name, Path::new("test_data/hello/src/main.rs"));
    assert_eq!(refs[0].line_start, 5);
    let refs = host.search("main").unwrap();
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].file_name, Path::new("test_data/hello/src/main.rs"));
    assert_eq!(refs[0].line_start, 5);

    let ids = host.search_for_id("name").unwrap();
    assert_eq!(ids.len(), 1);
    let id = ids[0];
    let def = host.get_def(id).unwrap();
    assert_eq!(def.name, "name");
    assert_eq!(def.kind, DefKind::Local);
    let refs = host.find_all_refs_by_id(id).unwrap();
    assert_eq!(refs.len(), 2);
    assert_eq!(refs[0].file_name, Path::new("test_data/hello/src/main.rs"));
    assert_eq!(refs[0].line_start, 1);
    assert_eq!(refs[1].file_name, Path::new("test_data/hello/src/main.rs"));
    assert_eq!(refs[1].line_start, 2);
    let refs = host.search("name").unwrap();
    assert_eq!(refs.len(), 2);
    assert_eq!(refs[0].file_name, Path::new("test_data/hello/src/main.rs"));
    assert_eq!(refs[0].line_start, 1);
    assert_eq!(refs[1].file_name, Path::new("test_data/hello/src/main.rs"));
    assert_eq!(refs[1].line_start, 2);
}

// TODO
// check span functions
// check complex programs
