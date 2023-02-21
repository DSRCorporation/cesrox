package cesrox.primitives

import uniffi.cesrox.*
import uniffi.cesrox.Signer as SignerNative

class Signer(private val value: SignerNative) {
    companion object {
        fun From(value: SignerNative): Signer = Signer(value)
        fun FromCodeAndRaw(code: MatterCodex, raw: List<UByte>, transferable: Boolean): Signer =
            Signer(signerNewWithCodeAndRaw(code, raw, transferable))

        fun FromQb64(qb64: String, transferable: Boolean): Signer = Signer(signerNewWithQb64(qb64, transferable))
        fun FromQb64b(qb64b: List<UByte>, transferable: Boolean): Signer =
            Signer(signerNewWithQb64b(qb64b, transferable))

        fun FromQb2(qb2: List<UByte>, transferable: Boolean): Signer = Signer(signerNewWithQb2(qb2, transferable))
    }

    fun code(): String {
        return signerCode(value)
    }

    fun raw(): List<UByte> {
        return signerRaw(value)
    }

    fun qb2(): List<UByte> {
        return signerQb2(value)
    }

    fun qb64b(): List<UByte> {
        return signerQb64b(value)
    }

    fun qb64(): String {
        return signerQb64(value)
    }
}

