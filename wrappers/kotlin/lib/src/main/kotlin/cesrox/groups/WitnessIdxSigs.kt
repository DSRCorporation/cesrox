package cesrox.groups

import cesrox.primitives.Siger
import uniffi.cesrox.*
import uniffi.cesrox.WitnessIdxSig as WitnessIdxSigNative
import uniffi.cesrox.WitnessIdxSigs as WitnessIdxSigsNative

class WitnessIdxSig {
    val value: WitnessIdxSigNative

    constructor(siger: Siger) {
        value = witnessIdsSigCreate(siger.value)
    }

    constructor(nativeValue: WitnessIdxSigNative) {
        value = nativeValue
    }

    val siger get() = Siger(value.siger)
}

class WitnessIdxSigs {
    val value: WitnessIdxSigsNative

    constructor(couples: List<WitnessIdxSig>) {
        value = witnessIdsSigsCreate(couples.map { it.value })
    }

    constructor(couples: WitnessIdxSigsNative) {
        value = couples
    }

    fun get(index: Int): WitnessIdxSig {
        return WitnessIdxSig(value.value[index])
    }

    fun qb2(): List<UByte> {
        return witnessIdsSigsQb2(value)
    }
    fun qb64b(): List<UByte> {
        return witnessIdsSigsQb64b(value)
    }
    fun qb64(): String {
        return witnessIdsSigsQb64(value)
    }
}