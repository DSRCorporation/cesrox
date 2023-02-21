package cesrox.primitives

import uniffi.cesrox.*
import uniffi.cesrox.Dater as DaterNative

class Dater(val value: uniffi.cesrox.Dater) {
    companion object {
        fun From(value: DaterNative): Dater = Dater(value)
        fun FromCodeAndRaw(code: MatterCodex, raw: List<UByte>): Dater =
            Dater(daterNewWithCodeAndRaw(code, raw))

        fun FromQb64(qb64: String): Dater = Dater(daterNewWithQb64(qb64))
        fun FromQb64b(qb64b: List<UByte>): Dater = Dater(daterNewWithQb64b(qb64b))
        fun FromQb2(qb2: List<UByte>): Dater = Dater(daterNewWithQb2(qb2))
    }

    fun code(): String {
        return daterCode(value)
    }

    fun raw(): List<UByte> {
        return daterRaw(value)
    }

    fun qb2(): List<UByte> {
        return daterQb2(value)
    }

    fun qb64b(): List<UByte> {
        return daterQb64b(value)
    }

    fun qb64(): String {
        return daterQb64(value)
    }
}

