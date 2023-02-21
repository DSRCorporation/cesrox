package cesrox.groups

import cesrox.primitives.Prefixer
import cesrox.primitives.Seqner
import cesrox.primitives.Saider
import uniffi.cesrox.*
import uniffi.cesrox.TransIdxSigGroup as TransIdxSigGroupNative
import uniffi.cesrox.TransIdxSigGroups as TransIdxSigGroupsNative

class TransIdxSigGroup {
    val value: TransIdxSigGroupNative

    constructor(prefixer: Prefixer, seqner: Seqner, saider: Saider, isigers: ControllerIdxSigs) {
        value = transIdxSigGroupCreate(prefixer.value, seqner.value, saider.value, isigers.value)
    }

    constructor(nativeValue: TransIdxSigGroupNative) {
        value = nativeValue
    }

    val prefixer get() = Prefixer(value.prefixer)

    val seqner get() = Seqner(value.seqner)

    val saider get() = Saider(value.saider)

    val isigers get() = ControllerIdxSigs(value.isigers)
}

class TransIdxSigGroups {
    val value: TransIdxSigGroupsNative

    constructor(couples: List<TransIdxSigGroup>) {
        value = transIdxSigGroupsCreate(couples.map { it.value })
    }

    constructor(couples: TransIdxSigGroupsNative) {
        value = couples
    }

    fun get(index: Int): TransIdxSigGroup {
        return TransIdxSigGroup(value.value[index])
    }

    fun qb2(): List<UByte> {
        return transIdxSigGroupsQb2(value)
    }
    fun qb64b(): List<UByte> {
        return transIdxSigGroupsQb64b(value)
    }
    fun qb64(): String {
        return transIdxSigGroupsQb64(value)
    }
}