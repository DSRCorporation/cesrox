package cesrox.primitives

import uniffi.cesrox.*
import uniffi.cesrox.Diger as DigerNative

class Diger(val value: DigerNative) {
    companion object {
        fun From(value: DigerNative): Diger = Diger(value)
        fun FromCodeAndRaw(code: MatterCodex, raw: List<UByte>): Diger =
            Diger(digerNewWithCodeAndRaw(code, raw))

        fun FromQb64(qb64: String): Diger = Diger(digerNewWithQb64(qb64))
        fun FromQb64b(qb64b: List<UByte>): Diger = Diger(digerNewWithQb64b(qb64b))
        fun FromQb2(qb2: List<UByte>): Diger = Diger(digerNewWithQb2(qb2))
    }

    fun code(): String {
        return digerCode(value)
    }

    fun raw(): List<UByte> {
        return digerRaw(value)
    }

    fun qb2(): List<UByte> {
        return digerQb2(value)
    }

    fun qb64b(): List<UByte> {
        return digerQb64b(value)
    }

    fun qb64(): String {
        return digerQb64(value)
    }
}

