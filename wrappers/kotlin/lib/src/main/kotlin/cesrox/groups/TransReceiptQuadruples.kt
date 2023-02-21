package cesrox.groups

import cesrox.primitives.Prefixer
import cesrox.primitives.Seqner
import cesrox.primitives.Saider
import cesrox.primitives.Siger
import uniffi.cesrox.*
import uniffi.cesrox.TransReceiptQuadruple as TransReceiptQuadrupleNative
import uniffi.cesrox.TransReceiptQuadruples as TransReceiptQuadruplesNative

class TransReceiptQuadruple {
    val value: TransReceiptQuadrupleNative

    constructor(prefixer: Prefixer, seqner: Seqner, saider: Saider, siger: Siger) {
        value = transReceiptQuadrupleCreate(prefixer.value, seqner.value, saider.value, siger.value)
    }

    constructor(nativeValue: TransReceiptQuadrupleNative) {
        value = nativeValue
    }

    val prefixer get() = Prefixer(value.prefixer)

    val seqner get() = Seqner(value.seqner)

    val saider get() = Saider(value.saider)

    val siger get() = Siger(value.siger)
}

class TransReceiptQuadruples {
    val value: TransReceiptQuadruplesNative

    constructor(couples: List<TransReceiptQuadruple>) {
        value = transReceiptQuadruplesCreate(couples.map { it.value })
    }

    constructor(couples: TransReceiptQuadruplesNative) {
        value = couples
    }

    fun get(index: Int): TransReceiptQuadruple {
        return TransReceiptQuadruple(value.value[index])
    }

    fun qb2(): List<UByte> {
        return transReceiptQuadruplesQb2(value)
    }
    fun qb64b(): List<UByte> {
        return transReceiptQuadruplesQb64b(value)
    }
    fun qb64(): String {
        return transReceiptQuadruplesQb64(value)
    }
}