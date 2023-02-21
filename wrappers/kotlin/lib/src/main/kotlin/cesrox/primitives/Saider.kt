package cesrox.primitives

import uniffi.cesrox.*
import uniffi.cesrox.Saider as SaiderNative

class Saider(val value: SaiderNative) {
    companion object {
        fun From(value: SaiderNative): Saider = Saider(value)
        fun FromQb64(qb64: String): Saider = Saider(saiderNewWithQb64(qb64))
        fun FromQb64b(qb64b: List<UByte>): Saider = Saider(saiderNewWithQb64b(qb64b))
        fun FromQb2(qb2: List<UByte>): Saider = Saider(saiderNewWithQb2(qb2))
    }
    
    fun code(): String {
        return saiderCode(value)
    }

    fun raw(): List<UByte> {
        return saiderRaw(value)
    }

    fun qb2(): List<UByte> {
        return saiderQb2(value)
    }

    fun qb64b(): List<UByte> {
        return saiderQb64b(value)
    }

    fun qb64(): String {
        return saiderQb64(value)
    }
}

