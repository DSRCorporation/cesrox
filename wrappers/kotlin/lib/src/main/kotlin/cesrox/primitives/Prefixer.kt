package cesrox.primitives

import uniffi.cesrox.*
import uniffi.cesrox.Prefixer as PrefixerNative

class Prefixer(val value: PrefixerNative) {
    companion object {
        fun From(value: PrefixerNative): Prefixer = Prefixer(value)
        fun FromCodeAndRaw(code: MatterCodex, raw: List<UByte>): Prefixer =
            Prefixer(prefixerNewWithCodeAndRaw(code, raw))

        fun FromQb64(qb64: String): Prefixer = Prefixer(prefixerNewWithQb64(qb64))
        fun FromQb64b(qb64b: List<UByte>): Prefixer = Prefixer(prefixerNewWithQb64b(qb64b))
        fun FromQb2(qb2: List<UByte>): Prefixer = Prefixer(prefixerNewWithQb2(qb2))
    }

    fun code(): String {
        return prefixerCode(value)
    }

    fun raw(): List<UByte> {
        return prefixerRaw(value)
    }

    fun qb2(): List<UByte> {
        return prefixerQb2(value)
    }

    fun qb64b(): List<UByte> {
        return prefixerQb64b(value)
    }

    fun qb64(): String {
        return prefixerQb64(value)
    }
}

