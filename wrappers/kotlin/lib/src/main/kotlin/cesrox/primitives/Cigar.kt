package cesrox.primitives

import uniffi.cesrox.*
import uniffi.cesrox.Cigar as CigarNative

class Cigar(val value: CigarNative) {
    companion object {
        fun From(value: CigarNative): Cigar = Cigar(value)
        fun FromCodeAndRaw(verfer: Verfer, code: MatterCodex, raw: List<UByte>): Cigar =
            Cigar(cigarNewWithCodeAndRaw(verfer.value, code, raw))

        fun FromQb64(verfer: Verfer, qb64: String): Cigar = Cigar(cigarNewWithQb64(verfer.value, qb64))
        fun FromQb64b(verfer: Verfer, Qb64b: List<UByte>): Cigar = Cigar(cigarNewWithQb64b(verfer.value, Qb64b))
        fun FromQb2(verfer: Verfer, qb2: List<UByte>): Cigar = Cigar(cigarNewWithQb2(verfer.value, qb2))
    }

    fun code(): String {
        return cigarCode(value)
    }

    fun raw(): List<UByte> {
        return cigarRaw(value)
    }

    fun qb2(): List<UByte> {
        return cigarQb2(value)
    }

    fun qb64b(): List<UByte> {
        return cigarQb64b(value)
    }

    fun qb64(): String {
        return cigarQb64(value)
    }
}

