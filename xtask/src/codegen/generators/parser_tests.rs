use crate::codegen::err::CodegenError;
use crate::codegen::generators::fmt::format_src;
use quote::{format_ident, quote};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn generate(root_path: impl AsRef<Path>) -> Result<String, CodegenError> {
    let success_test_root_path = "data/tests/parser/success";
    let success_test_path = root_path.as_ref().join(success_test_root_path);
    let success_test_cases = TestCase::list(&success_test_path, success_test_root_path)?
        .into_iter()
        .map(|case| case.generate("parser_success"))
        .collect::<Vec<_>>();

    let error_test_root_path = "data/tests/parser/error";
    let error_test_path = root_path.as_ref().join(error_test_root_path);
    let error_test_cases = TestCase::list(&error_test_path, error_test_root_path)?
        .into_iter()
        .map(|case| case.generate("parser_error"))
        .collect::<Vec<_>>();

    let tests = quote! {
        mod common;

        use common::parse;
        use expect_test::expect_file;
        use libsakura::parser::EntryPoint;
        use std::fs;
        use std::path::Path;

        #(#success_test_cases)*
        #(#error_test_cases)*
    };

    let result = format_src(&tests.to_string())?;
    Ok(result)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TestCase {
    source: PathBuf,
    syntax: PathBuf,
}

impl TestCase {
    pub fn list(
        test_case_path: impl AsRef<Path>,
        entry_path_root: impl AsRef<Path>,
    ) -> anyhow::Result<Vec<TestCase>> {
        let path_entries = fs::read_dir(test_case_path.as_ref())?.collect::<Result<Vec<_>, _>>()?;
        let mut test_cases = Vec::new();

        for entry in path_entries {
            let entry_path = entry.path();

            if entry_path.extension().unwrap_or_default() != "sk" {
                continue;
            }

            let entry_path = entry_path_root
                .as_ref()
                .join(entry_path.file_name().expect("unable to get file name"));

            test_cases.push(TestCase {
                source: entry_path.clone(),
                syntax: entry_path.with_extension("skast"),
            });
        }

        test_cases.sort();

        Ok(test_cases)
    }

    pub fn generate(self, prefix: &str) -> impl quote::ToTokens {
        let test_name = format_ident!(
            "{}_{}",
            prefix,
            self.source.file_stem().and_then(OsStr::to_str).expect("unable to get file stem")
        );
        let source_path = self.source.to_str().expect("unable to convert source path to string");
        let syntax_path = self.syntax.to_str().expect("unable to convert syntax path to string");

        quote! {
            #[test]
            fn #test_name() {
                let root = Path::new(env!("CARGO_MANIFEST_DIR"));
                let source = fs::read_to_string(root.join(#source_path)).expect("unable to read source file");
                let (result, _has_errors) = parse(EntryPoint::SourceFile, &source);
                let expected = expect_file!(root.join(#syntax_path));

                expected.assert_eq(&result);
            }
        }
    }
}
