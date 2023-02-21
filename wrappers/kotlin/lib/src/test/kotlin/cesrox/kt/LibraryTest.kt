package cesrox

import cesrox.groups.NonTransReceiptCouple
import cesrox.groups.NonTransReceiptCouples
import cesrox.CesrGroup
import cesrox.primitives.Cigar
import cesrox.primitives.MatterCodex
import cesrox.primitives.Verfer
import org.junit.jupiter.api.Test
import uniffi.cesrox.CustomPayload

class LibraryTest {
    @Test
    fun parseMessageListFromStreamTest() {
        var bytes: List<UByte> =
            "{\"v\":\"KERI10JSON00012b_\",\"t\":\"icp\"}-CABBD8-gMSJ6K1PQ7_gG5ZJn2NkHQJgdkiNrTBz_FWWS_cC0BDc1i44ZX0jaIHh5oNDx-TITbPnI6VEn2nKlqPwkkTF452X7XxYh80tolDpReYwZpnD8TF4Or2v3CpSCikyt6EG{\"v\":\"KERI10JSON00012b_\",\"t\":\"icp\"}-CABBD8-gMSJ6K1PQ7_gG5ZJn2NkHQJgdkiNrTBz_FWWS_cC0BDc1i44ZX0jaIHh5oNDx-TITbPnI6VEn2nKlqPwkkTF452X7XxYh80tolDpReYwZpnD8TF4Or2v3CpSCikyt6EG".toByteArray()
                .map { it.toUByte() }
        val messageList = MessageList.fromBytes(bytes)
        var message1 = messageList.messages[0]
        var message2 = messageList.messages[1]
        var message3 = messageList.messages[2]
        var message4 = messageList.messages[3]

        var payload1: CustomPayload = message1.payload!!
        var grop1: CesrGroup = message2.group!!
        var payload2: CustomPayload = message3.payload!!
        var grop2: CesrGroup = message4.group!!

        var nonTransReceiptCouples = grop1.nonTransReceiptCouples!!
        var nonTransReceiptCouple = nonTransReceiptCouples.get(0)

        println("message1: $message1")
        println("message2: $message2")
        println("message3: $message3")
        println("message4: $message4")

        println("payload1: ${payload1.value}")
        println("grop1: $grop1")
        println("payload2: ${payload2.value}")
        println("grop2: $grop2")
        println("nonTransReceiptCouple: $nonTransReceiptCouple")
        println("cigar: ${nonTransReceiptCouple.cigar.qb64()}")
        println("verfer: ${nonTransReceiptCouple.verfer.qb64()}")
    }

    @Test
    fun createGroupTest() {
        val verferBytes = "abcdefghijklmnopqrstuvwxyz012345".toByteArray().map { it.toUByte() }
        val cigarQb64 = "0BCdI8OSQkMJ9r-xigjEByEjIua7LHH3AOJ22PQKqljMhuhcgh9nGRcKnsz5KvKd7K_H9-1298F4Id1DxvIoEmCQ"

        val verfer = Verfer.FromCodeAndRaw(MatterCodex.ED25519, verferBytes)
        val cigar = Cigar.FromQb64(verfer, cigarQb64)
        println("cigar: ${cigar.qb64()}")

        val group = NonTransReceiptCouples(listOf(NonTransReceiptCouple(cigar)))
        println("group: ${group.qb64()}")
    }
}
