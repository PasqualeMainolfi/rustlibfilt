// TODO: Chebisev

pub enum BiquadFilterType {
    Lp,
    Hp,
    Bp0dB,
    Bpsg,
    Notch,
    Ap,
    Peq,
    LpShelf,
    HpShelf
}

pub enum TwoZeroTwoPoleFilterType {
    Notch,
    Bp,
}
pub enum HarmonicFilterType {
    CombFIR,
    CombFreeverbFIR,
    CombIIR,
    LPFBCombFilter,
    Allpass,
    AllpassFreeverb,
    LPFBAllpassFilter
}

pub enum OnePoleFilterType {
    LowPass,
    HighPass
}

pub enum DcBlockFilterType {
    DcBlockJulius
}

pub enum NarrowFilterType {
    Bp,
    Notch
}

pub enum ZavalishinFilterType {
    OnePoleZeroDelay,
    NaiveOnePole,
    TrapIntOnePole,
    StateVariable
}

pub enum ButterFilterType {
    Lp,
    Hp,
    Bp,
    Notch
}

pub enum FilterType {
    BiquadType(BiquadFilterType),
    TwoZeroTwoPoleType(TwoZeroTwoPoleFilterType),
    HarmonicType(HarmonicFilterType),
    OnePoleType(OnePoleFilterType),
    DcBlockType(DcBlockFilterType),
    NarrowType(NarrowFilterType),
    ZavalishinType(ZavalishinFilterType),
    ButterType(ButterFilterType)
}