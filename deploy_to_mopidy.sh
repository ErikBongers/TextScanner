#!/bin/bash

git pull

cd ./text_scanner_py || exit

maturin build

cp ../target/maturin/libtext_scanner_py.so ~/programming/mopidy/mopidy_eboback/mopidy_eboback/lib/text_scanner_py.so