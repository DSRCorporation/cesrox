package cesrox

import uniffi.cesrox.Message as MessageNative

class Message(val value: MessageNative) {
    val payload get() = value.payload

    val group
        get() = if (value.group != null) {
            CesrGroup(value.group!!)
        } else {
            null
        }
}