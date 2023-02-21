package cesrox.primitives

import uniffi.cesrox.*
import uniffi.cesrox.Seqner as SeqnerNative

class Seqner(val value: SeqnerNative) {
    companion object {
        fun From(value: SeqnerNative): Seqner = Seqner(value)
        fun FromCodeAndRaw(code: MatterCodex, raw: List<UByte>): Seqner =
            Seqner(seqnerNewWithCodeAndRaw(code, raw))

        fun FromQb64(qb64: String): Seqner = Seqner(seqnerNewWithQb64(qb64))
        fun FromQb64b(qb64b: List<UByte>): Seqner = Seqner(seqnerNewWithQb64b(qb64b))
        fun FromQb2(qb2: List<UByte>): Seqner = Seqner(seqnerNewWithQb2(qb2))
    }

    fun code(): String {
        return seqnerCode(value)
    }

    fun raw(): List<UByte> {
        return seqnerRaw(value)
    }

    fun qb2(): List<UByte> {
        return seqnerQb2(value)
    }

    fun qb64b(): List<UByte> {
        return seqnerQb64b(value)
    }

    fun qb64(): String {
        return seqnerQb64(value)
    }
}

