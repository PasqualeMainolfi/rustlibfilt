from rustlibfilt import Biquad, Harmonic, OnePole, Narrow, TwoZeroTwoPole, Zavalishin, Butter
import numpy as np


class Rustlibfilt():
    def __init__(self, fs: int, family: str, mode: str, buffer_lenght: int|None = None, order: int|None = None) -> None:
        
        """
        INIT FILTER
        
        Args:
        -----
            fs: int
                sampling rate
                
            family: str
                filter family: [biquad, harmonic, onePole, narrow, twoZeroTwoPole, zavalishin, butter]
                
            mode: str
                filter type (for family):
                    - biquad: [lp, hp, bp0db, bpsg, notch, ap, peq, lpshelf, hpshelf]
                    - harmonic: [combf, combi, allpass, allpassfreev, lpcombi, lpallpass]
                    - onepole: [lp, hp]
                    - narrow: [bp, notch]
                    - twozerotwopole: [bp, notch]
                    - zavalishin: [zdf, naive, trap, svf]
                    - butter: [lp, hp, bp, br]
                    
            buffer_length: int|None
                delay buffer length (for harmonic family)
                
            order: int|None
                order of filter (for onepole, narrow, butter)
        """
        
        self.fs = fs
        self.family = family
        self.filter_type = None
        self.mode = mode
        self.coeffs = None
        n_order = 1 if order is None else order

        match family:
            case "biquad":
                assert self.mode in ["lp", "hp", "bp0db", "bpsg", "notch", "ap", "peq", "lpshelf", "hpshelf"], f"\n[ERROR] Wrong filter mode {self.mode} for {family} family!\n"
                self.filter_type = Biquad(fs=self.fs)
            case "harmonic":
                assert self.mode in ["combf", "combi", "allpass", "allpassfreev", "lpcombi", "lpallpass"], f"\n[ERROR] Wrong filter mode {self.mode} for {family} family!\n"
                self.filter_type = Harmonic(mode=self.mode, buffer_delay=buffer_lenght, fs=self.fs)
            case "onepole":
                assert self.mode in ["lp", "hp"], f"\n[ERROR] Wrong filter mode {self.mode} for {family} family!\n"
                self.filter_type = OnePole(fs=self.fs, order=n_order)
            case "narrow":
                assert self.mode in ["bp", "notch"], f"\n[ERROR] Wrong filter mode {self.mode} for {family} family!\n"
                self.filter_type = Narrow(fs=self.fs, order=n_order)
            case "twozerotwopole":
                assert self.mode in ["bp", "notch"], f"\n[ERROR] Wrong filter mode {self.mode} for {family} family!\n"
                self.filter_type = TwoZeroTwoPole(fs=self.fs)
            case "zavalishin":
                assert self.mode in ["zdf", "naive", "trap", "svf"], f"\n[ERROR] Wrong filter mode {self.mode} for {family} family!\n"
                self.filter_type = Zavalishin(fs=self.fs)
            case "butter":
                assert self.mode in ["lp", "hp", "bp", "br"], f"\n[ERROR] Wrong filter mode {self.mode} for {family} family!\n"
                self.filter_type = Butter(fs=self.fs, order=n_order)
            case _:
                print(f"[ERROR] Wrong family {family} type!\n")
                exit(1)
    
    def design_filter(self, fc: float|None = None, q: float|None = None, dbgain: float|None = None, t60: float|None = None, bw: float|None = None, fc_spread: float|None = None, fc_lp: float|None = None) -> None:
        
        """
        DESIGN FILTER
        
        Args:
        -----
            fc: float
                cut off frequency
                
            q: float|None
                Q factor (for biquad)
                
            dbgain: float|None
                 dB value for peaking and shelf filters (for biquad)
                 
            t60: float|None
                reverb time in s. (for harmonic)
                
            bw: float|None
                band width (for narrow, twozerotwopole and butter)
                
            fc_spread: float|None
                value in Hz, in zavalishin svf fhigh = fc + fc_spread, bw = fhigh - fc
            
            fc_lp: float|None
                low pass cut off frequency in harmonic family for lpcombi and lpallpass only 
        """
        
        coeffs = None
        match self.family:
            case "biquad":
                coeffs = self.filter_type.design_filter(mode=self.mode, fc=fc, q=q, dbgain=dbgain)
            case "harmonic":
                self.filter_type.design_filter(t60=t60, fc=fc_lp)
            case "onepole":
                coeffs = self.filter_type.design_filter(mode=self.mode, fc=fc)
            case "narrow":
                coeffs = self.filter_type.design_filter(mode=self.mode, fc=fc, bw=bw)
            case "twozerotwopole":
                coeffs = self.filter_type.design_filter(mode=self.mode, fc=fc, bw=bw)
            case "zavalishin":
                self.filter_type.design_filter(mode=self.mode, fc=fc, fc_spread=fc_spread)
            case "butter":
                coeffs = self.filter_type.design_filter(mode=self.mode, fc=fc, bw=bw)
            case _:
                print(f"[ERROR] Wrong family {self.family} type!\n")
                exit(1)
        
        if coeffs is not None:
            self.coeffs = np.array(list(coeffs), dtype=np.float64)
    
    def filt_sample(self, sample: float) -> float:
        
        """
        APPLY FILTER SAMPLE BY SAMPLE

        Args:
        -----
            sample: float
                sample in

        Returns
        -------
        float
            filtered sample
        """
        
        y: float 
        if self.coeffs is not None:
            y = self.filter_type.filt_sample(sample=sample, coeffs=tuple(self.coeffs))
        else:
            y = self.filter_type.filt_sample(sample=sample)
        return y
    
    def filt_frame(self, frame: np.ndarray) -> np.ndarray:
        
        """
        APPLY FILTER ON AUDIO FRAME
        ...without reset cache of delayed samples.
        If you want reset cache, call clear_delayed_samples_cache!

        Args:
        -----
            sample: float
                sample in

        Returns
        -------
        np.ndarray
            filtered frame
        """
        
        y: np.ndarray 
        if self.coeffs is not None:
            y = self.filter_type.filt_frame(frame=frame, coeffs=tuple(self.coeffs))
        else:
            y = self.filter_type.filt_frame(frame=frame)
        return y
    
    def clear_delayed_samples_cache(self) -> None:
        
        """
        CLEAR DELAYED SAMPLES CACHE
        
        clear samples history, set to zero the value of delayed samples
        """
        
        self.filter_type.clear_delayed_samples_cache()
        
        
        
        
        
                
        