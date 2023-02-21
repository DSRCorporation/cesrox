package cesrox.groups

import cesrox.primitives.Cigar
import cesrox.primitives.Verfer
import uniffi.cesrox.*
import uniffi.cesrox.NonTransReceiptCouple as NonTransReceiptCoupleNative
import uniffi.cesrox.NonTransReceiptCouples as NonTransReceiptCouplesNative

class NonTransReceiptCouple {
    val value: NonTransReceiptCoupleNative

    constructor(cigar: Cigar) {
        value = nonTransReceiptCoupleCreate(cigar.value)
    }

    constructor(nativeValue: NonTransReceiptCoupleNative) {
        value = nativeValue
    }

    val cigar get() = Cigar(value.cigar)
    val verfer get() = Verfer(value.cigar.verfer)
}

class NonTransReceiptCouples {
    val value: NonTransReceiptCouplesNative

    constructor(couples: List<NonTransReceiptCouple>) {
        value = nonTransReceiptCouplesCreate(couples.map { it.value })
    }

    constructor(couples: NonTransReceiptCouplesNative) {
        value = couples
    }

    fun get(index: Int): NonTransReceiptCouple {
        return NonTransReceiptCouple(value.value[index])
    }

    fun qb2(): List<UByte> {
        return nonTransReceiptCouplesQb2(value)
    }
    fun qb64b(): List<UByte> {
        return nonTransReceiptCouplesQb64b(value)
    }
    fun qb64(): String {
        return nonTransReceiptCouplesQb64(value)
    }
}