package cesrox.groups

import cesrox.primitives.Prefixer
import uniffi.cesrox.*
import uniffi.cesrox.TransLastIdxSigGroup as TransLastIdxSigGroupNative
import uniffi.cesrox.TransLastIdxSigGroups as TransLastIdxSigGroupsNative

class TransLastIdxSigGroup {
    val value: TransLastIdxSigGroupNative

    constructor(prefixer: Prefixer, isigers: ControllerIdxSigs) {
        value = transLastIdxSigGroupCreate(prefixer.value, isigers.value)
    }

    constructor(nativeValue: TransLastIdxSigGroupNative) {
        value = nativeValue
    }

    val prefixer get() = Prefixer(value.prefixer)

    val isigers get() = ControllerIdxSigs(value.isigers)
}

class TransLastIdxSigGroups {
    val value: TransLastIdxSigGroupsNative

    constructor(couples: List<TransLastIdxSigGroup>) {
        value = transLastIdxSigGroupsCreate(couples.map { it.value })
    }

    constructor(couples: TransLastIdxSigGroupsNative) {
        value = couples
    }

    fun get(index: Int): TransLastIdxSigGroup {
        return TransLastIdxSigGroup(value.value[index])
    }

    fun qb2(): List<UByte> {
        return transLastIdxSigGroupsQb2(value)
    }
    fun qb64b(): List<UByte> {
        return transLastIdxSigGroupsQb64b(value)
    }
    fun qb64(): String {
        return transLastIdxSigGroupsQb64(value)
    }
}