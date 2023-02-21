package cesrox

import uniffi.cesrox.*

class MessageList(val rest: List<UByte>, val messages: List<Message>) {
    companion object {
        fun fromBytes(bytes: List<UByte>): MessageList {
            val result: MessageListFromStreamResult = messageListFromStreamBytes(bytes)
            return MessageList(result.rest, result.messages.map{ Message(it) })
        }
    }
}