## Cesrox Uni-FFI
`Cesrox` library bindings based on [uniffi-rs](https://github.com/mozilla/uniffi-rs).

### Kotlin
```
cargo run --bin uniffi-bindgen generate src/cesrox.udl --language kotlin
```

**Kotlin Usage Example**
```
import uniffi.cesrox.*

fun main(args: Array<String>) {
    var keyBytes: List<UByte> = listOf(118, 206, 49, 173, 1, 174, 101, 84, 226, 178, 184, 205, 80, 1, 218, 21, 29, 184, 40, 50, 28, 119, 62, 15, 96, 72, 64, 37, 59, 79, 9, 184) .map { it.toUByte() }
    var key = keyCreate(keyBytes);
    var basicPrefix = basicPrefixCreate(BasicCode.ED25519, key)

    var basicPrefixString = basicPrefixToStr(basicPrefix)
    println("BasicPrefixString: $basicPrefixString")
    var basicPrefixRestoredFromString = basicPrefixFromStr(basicPrefixString)
    println("BasicPrefixFromString: $basicPrefixRestoredFromString")

    var basicPrefixBytes = basicPrefixToBytes(basicPrefix)
    println("BasicPrefixBytes: $basicPrefixBytes")
    var basicPrefixRestored = basicPrefixFromBytes(basicPrefixBytes)
    println("BasicPrefixRestored: $basicPrefixRestored")
    println("BasicPrefixBytesDerivation: ${basicPrefixRestored.derivation}")
    println("BasicPrefixBytesPublicKey: ${basicPrefixRestored.publicKey.value}")

    var indexedControllerSignatures: List<UByte> = listOf(45, 65, 65, 66, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65) .map { it.toUByte() }
    var messageFromStreamBytesResult = messageFromStreamBytes(indexedControllerSignatures)
    println("messageFromStreamBytesResult rest: ${messageFromStreamBytesResult.rest}")
    println("messageFromStreamBytesResult: ${messageFromStreamBytesResult.message}")
}
```

### Python
```
cargo run --bin uniffi-bindgen generate src/cesrox.udl --language python
```
