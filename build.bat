@ECHO OFF
cargo update --quiet
cargo build --target-dir build --release
@ECHO ON
getver.py
@ECHO OFF
MOVE build\release\lucerna.exe build >nul
@RD /S /Q "build\release"
DEL build\.rustc_info.json
echo Done
