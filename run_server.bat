@echo off
echo Attempting to start Archiva Server...
cargo +nightly -C ./archiva-server/ run -Z unstable-options

echo.
pause