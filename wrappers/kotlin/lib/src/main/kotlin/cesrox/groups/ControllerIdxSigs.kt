package cesrox.groups

import cesrox.primitives.Siger
import uniffi.cesrox.*
import uniffi.cesrox.ControllerIdxSig as ControllerIdxSigNative
import uniffi.cesrox.ControllerIdxSigs as ControllerIdxSigsNative

class ControllerIdxSig {
    val value: ControllerIdxSigNative

    constructor(siger: Siger) {
        value = controllerIdxSigCreate(siger.value)
    }

    constructor(nativeValue: ControllerIdxSigNative) {
        value = nativeValue
    }

    val siger get() = Siger(value.siger)
}

class ControllerIdxSigs {
    val value: ControllerIdxSigsNative

    constructor(couples: List<ControllerIdxSig>) {
        value = controllerIdxSigsCreate(couples.map { it.value })
    }

    constructor(couples: ControllerIdxSigsNative) {
        value = couples
    }

    fun get(index: Int): ControllerIdxSig {
        return ControllerIdxSig(value.value[index])
    }

    fun qb2(): List<UByte> {
        return controllerIdxSigsQb2(value)
    }
    fun qb64b(): List<UByte> {
        return controllerIdxSigsQb64b(value)
    }
    fun qb64(): String {
        return controllerIdxSigsQb64(value)
    }
}