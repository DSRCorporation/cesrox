package cesrox.groups

import cesrox.primitives.Seqner
import cesrox.primitives.Dater
import uniffi.cesrox.*
import uniffi.cesrox.FirstSeenReplayCouple as FirstSeenReplayCoupleNative
import uniffi.cesrox.FirstSeenReplayCouples as FirstSeenReplayCouplesNative

class FirstSeenReplayCouple {
    val value: FirstSeenReplayCoupleNative

    constructor(firner: Seqner, dater: Dater) {
        value = firstSeenReplayCoupleCreate(firner.value, dater.value)
    }

    constructor(nativeValue: FirstSeenReplayCoupleNative) {
        value = nativeValue
    }

    val firner get() = Seqner(value.firner)

    val dater get() = Dater(value.dater)
}

class FirstSeenReplayCouples {
    val value: FirstSeenReplayCouplesNative

    constructor(couples: List<FirstSeenReplayCouple>) {
        value = firstSeenReplayCouplesCreate(couples.map { it.value })
    }

    constructor(couples: FirstSeenReplayCouplesNative) {
        value = couples
    }

    fun get(index: Int): FirstSeenReplayCouple {
        return FirstSeenReplayCouple(value.value[index])
    }

    fun qb2(): List<UByte> {
        return firstSeenReplayCouplesQb2(value)
    }

    fun qb64b(): List<UByte> {
        return firstSeenReplayCouplesQb64b(value)
    }

    fun qb64(): String {
        return firstSeenReplayCouplesQb64(value)
    }
}