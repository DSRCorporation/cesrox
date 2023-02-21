package cesrox.primitives

import uniffi.cesrox.*
import uniffi.cesrox.Siger as SigerNative

class Siger(val value: SigerNative) {
    companion object {
        fun From(value: SigerNative): Siger = Siger(value)
        fun FromCodeAndRaw(code: IndexerCodex, raw: List<UByte>, index: UInt, ondex: UInt?): Siger =
            Siger(sigerNewWithCodeAndRaw(code, raw, index, ondex))

        fun FromQb64(qb64: String): Siger = Siger(sigerNewWithQb64(qb64))
        fun FromQb64b(qb64b: List<UByte>): Siger = Siger(sigerNewWithQb64b(qb64b))
        fun FromQb2(qb2: List<UByte>): Siger = Siger(sigerNewWithQb2(qb2))
    }

    fun code(): String {
        return sigerCode(value)
    }

    fun raw(): List<UByte> {
        return sigerRaw(value)
    }

    fun qb2(): List<UByte> {
        return sigerQb2(value)
    }

    fun qb64b(): List<UByte> {
        return sigerQb64b(value)
    }

    fun qb64(): String {
        return sigerQb64(value)
    }
}

