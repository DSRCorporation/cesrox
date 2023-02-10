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
    var stream: List<UByte> = "{\"v\":\"KERI10JSON00012b_\",\"t\":\"icp\"}-CABBD8-gMSJ6K1PQ7_gG5ZJn2NkHQJgdkiNrTBz_FWWS_cC0BDc1i44ZX0jaIHh5oNDx-TITbPnI6VEn2nKlqPwkkTF452X7XxYh80tolDpReYwZpnD8TF4Or2v3CpSCikyt6EG{\"v\":\"KERI10JSON00012b_\",\"t\":\"icp\"}-CABBD8-gMSJ6K1PQ7_gG5ZJn2NkHQJgdkiNrTBz_FWWS_cC0BDc1i44ZX0jaIHh5oNDx-TITbPnI6VEn2nKlqPwkkTF452X7XxYh80tolDpReYwZpnD8TF4Or2v3CpSCikyt6EG".toByteArray().map { it.toUByte() }
    var result: MessageListFromStreamResult = messageListFromStreamBytes(stream)
    var message1 = result.messages[0]
    var message2 = result.messages[1]
    var message3 = result.messages[2]
    var message4 = result.messages[3]

    var payload1: CustomPayload = message1.payload!!
    var grop1: CesrGroup = message2.group!!
    var payload2: CustomPayload = message3.payload!!
    var grop2: CesrGroup = message4.group!!

    var nonTransReceiptCouples: NonTransReceiptCouples = grop1.nonTransReceiptCouples!!
    var nonTransReceiptCouple: NonTransReceiptCouple = nonTransReceiptCouples.value.get(0)


    println("message1: $message1")
    println("message2: $message2")
    println("message3: $message3")
    println("message4: $message4")

    println("payload1: $payload1")
    println("grop1: $grop1")
    println("payload2: $payload2")
    println("grop2: $grop2")

    println("nonTransReceiptCouple cigar: ${nonTransReceiptCouple.cigar}")
    println("nonTransReceiptCouple verfer: ${nonTransReceiptCouple.verfer}")

    var matter: Matter = matterNewWithQb64("BGlOiUdp5sMmfotHfCWQKEzWR91C72AH0lT84c0um-Qj")
    var values = listOf<Matter>(matter)
    var controllerIdxSigs = controllerIdxSigsCreate(values)
    var string = controllerIdxSigsQb64(controllerIdxSigs)
    println("controllerIdxSigsQb64: $string")   
}
```

### Python
```
cargo run --bin uniffi-bindgen generate src/cesrox.udl --language python
```
