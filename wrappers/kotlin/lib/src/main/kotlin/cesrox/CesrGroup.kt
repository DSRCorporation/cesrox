package cesrox

import cesrox.groups.AttachedMaterialQuadlets
import cesrox.groups.ControllerIdxSigs
import cesrox.groups.FirstSeenReplayCouples
import cesrox.groups.NonTransReceiptCouples
import cesrox.groups.SealSourceCouples
import cesrox.groups.TransIdxSigGroups
import cesrox.groups.TransLastIdxSigGroups
import cesrox.groups.TransReceiptQuadruples
import cesrox.groups.WitnessIdxSigs
import uniffi.cesrox.CesrGroup as CesrGroupNative

class CesrGroup(val value: CesrGroupNative) {
    val attachedMaterialQuadlets
        get() =
            if (value.attachedMaterialQuadlets != null) AttachedMaterialQuadlets(value.attachedMaterialQuadlets!!) else null

    val controllerIdxSigs
        get() =
            if (value.controllerIdxSigs != null) ControllerIdxSigs(value.controllerIdxSigs!!) else null


    val firstSeenReplayCouples
        get() =
            if (value.firstSeenReplayCouples != null) FirstSeenReplayCouples(value.firstSeenReplayCouples!!) else null


    val nonTransReceiptCouples
        get() =
            if (value.nonTransReceiptCouples != null) NonTransReceiptCouples(value.nonTransReceiptCouples!!) else null

    val sealSourceCouples
        get() =
            if (value.sealSourceCouples != null) SealSourceCouples(value.sealSourceCouples!!) else null

    val transIdxSigGroups
        get() =
            if (value.transIdxSigGroups != null) TransIdxSigGroups(value.transIdxSigGroups!!) else null

    val transLastIdxSigGroups
        get() =
            if (value.transLastIdxSigGroups != null) TransLastIdxSigGroups(value.transLastIdxSigGroups!!) else null

    val transReceiptQuadruples
        get() =
            if (value.transReceiptQuadruples != null) TransReceiptQuadruples(value.transReceiptQuadruples!!) else null

    val witnessIdxSigs
        get() =
            if (value.witnessIdxSigs != null) WitnessIdxSigs(value.witnessIdxSigs!!) else null

    val pathedMaterialQuadlets get() = null
    val sadPathSigs get() = null
    val sadPathSigGroups get() = null
}