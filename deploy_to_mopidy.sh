#!/bin/bash

git pull

cd ./text_scanner_py || exit

maturin build

cp ../target/maturin/libtext_scaner_py.so ~/programming/mopidy/mopidy_eboback/mopidy_eboback/lib/text_scaner_py.so