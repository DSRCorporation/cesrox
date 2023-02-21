package cesrox.groups

import cesrox.primitives.Seqner
import cesrox.primitives.Saider
import uniffi.cesrox.*
import uniffi.cesrox.SealSourceCouple as SealSourceCoupleNative
import uniffi.cesrox.SealSourceCouples as SealSourceCouplesNative

class SealSourceCouple {
    val value: SealSourceCoupleNative

    constructor(seqner: Seqner, saider: Saider) {
        value = sealSourceCoupleCreate(seqner.value, saider.value)
    }

    constructor(nativeValue: SealSourceCoupleNative) {
        value = nativeValue
    }

    val seqner get() = Seqner(value.seqner)

    val saider get() = Saider(value.saider)
}

class SealSourceCouples {
    val value: SealSourceCouplesNative

    constructor(couples: List<SealSourceCouple>) {
        value = sealSourceCouplesCreate(couples.map { it.value })
    }

    constructor(couples: SealSourceCouplesNative) {
        value = couples
    }

    fun get(index: Int): SealSourceCouple {
        return SealSourceCouple(value.value[index])
    }

    fun qb2(): List<UByte> {
        return sealSourceCouplesQb2(value)
    }
    fun qb64b(): List<UByte> {
        return sealSourceCouplesQb64b(value)
    }
    fun qb64(): String {
        return sealSourceCouplesQb64(value)
    }
}