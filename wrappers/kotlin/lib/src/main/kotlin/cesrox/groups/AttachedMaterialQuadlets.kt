package cesrox.groups

import cesrox.CesrGroup
import uniffi.cesrox.*
import uniffi.cesrox.AttachedMaterialQuadlets as AttachedMaterialQuadletsNative

class AttachedMaterialQuadlets {
    val value: AttachedMaterialQuadletsNative

    constructor(couples: List<CesrGroup>) {
        value = attachedMaterialQuadletsCreate(couples.map { it.value })
    }

    constructor(couples: AttachedMaterialQuadletsNative) {
        value = couples
    }

    fun get(index: Int): CesrGroup {
        return CesrGroup(value.value[index])
    }

    fun qb2(): List<UByte> {
        return attachedMaterialQuadletsQb2(value)
    }
    fun qb64b(): List<UByte> {
        return attachedMaterialQuadletsQb64b(value)
    }
    fun qb64(): String {
        return attachedMaterialQuadletsQb64(value)
    }
}