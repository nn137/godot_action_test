cd ../rust
echo %GODOT4_BIN%
cargo clean
@REM cargo build
cargo build --release
cd ../godot_action_test
@REM %GODOT4_BIN% --headless --verbose --export-debug "Windows Desktop" ../build/GodotActionTest_Debug.exe
%GODOT4_BIN% --headless --verbose --export-release "Windows Desktop" ../build/GodotActionTest.exe
