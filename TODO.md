https://github.com/rust-lang/rust/pull/121865/files
3ab693689ae878aca6c0c6f188370de973d246e1~
tests/mir-opt/unnamed-fields

./x.py test -j 15 --stage 1 tests/mir-opt/unnamed-fields/field_access.rs
./x.py test --stage 1 tests/mir-opt/copy-prop
./x.py test -j 10 --stage 1 tests/mir-opt/copy-prop/branch.rs

./x.py test --stage 1 src/test/mir-opt
./x.py test --stage 1 src/test/mir-opt/path/to/your_test.rs
./x.py test --stage 1 src/test/mir-opt --verbose
./x.py test --bless
./x.py test -j 8 --stage 1 src/test/mir-opt
./x.py test --stage 1 src/test/mir-opt --keep-stage 1

