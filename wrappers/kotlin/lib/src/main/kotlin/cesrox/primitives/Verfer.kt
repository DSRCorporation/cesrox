package cesrox.primitives

import uniffi.cesrox.*
import uniffi.cesrox.Verfer as VerferNative

class Verfer(val value: VerferNative) {
    companion object {
        fun From(value: VerferNative): Verfer = Verfer(value)
        fun FromCodeAndRaw(code: MatterCodex, raw: List<UByte>): Verfer =
            Verfer(verferNewWithCodeAndRaw(code, raw))

        fun FromQb64(qb64: String): Verfer = Verfer(verferNewWithQb64(qb64))
        fun FromQb64b(qb64b: List<UByte>): Verfer = Verfer(verferNewWithQb64b(qb64b))
        fun FromQb2(qb2: List<UByte>): Verfer = Verfer(verferNewWithQb2(qb2))
    }

    fun code(): String {
        return verferCode(value)
    }

    fun raw(): List<UByte> {
        return verferRaw(value)
    }

    fun qb2(): List<UByte> {
        return verferQb2(value)
    }

    fun qb64b(): List<UByte> {
        return verferQb64b(value)
    }

    fun qb64(): String {
        return verferQb64(value)
    }
}

