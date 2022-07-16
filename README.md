# floui-rs ios standalone

This is just an example of a pure rust app built for iOS using floui and the objc crate, with no Objective-C code. 

## Build
```
cargo bundle --target x86_64-apple-ios
xcrun simctl install booted target/x86_64-apple-ios/debug/bundle/ios/pure.app
xcrun simctl launch --console booted com.neurosrg.pure
```
