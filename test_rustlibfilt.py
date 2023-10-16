# import section
from rustlibfilt import Biquad, Harmonic, OnePole, Narrow, TwoZeroTwoPole, Zavalishin, Butter
import librosa as lb
import soundfile as sf
import numpy as np

# main scripts

PATH = "./audio_files/vox.wav"
SIG, SR = lb.load(PATH, sr=None)

# main function
def main() -> None:
    bq = Biquad(fs=SR)
    coeffs = bq.design_filter(mode="peq", fc=300, q=0.5, dbgain=-12)
    
    del_sample = int(0.037 * SR)
    comb1 = Harmonic(mode="lpcombi", buffer_delay=del_sample, fs=SR)
    comb1.design_filter(t60=1.5, fc=100)
    
    allpass = Harmonic(mode="lpallpass", buffer_delay=del_sample, fs=SR)
    allpass.design_filter(t60=1.5, fc=1500)
    
    lp_onepole = OnePole(fs=SR, order=3)
    lp_onepole_coeffs = lp_onepole.design_filter(mode="lp", fc=100)
    print(lp_onepole_coeffs)
    
    hp_onepole = OnePole(fs=SR)
    hp_onepole_coeffs = hp_onepole.design_filter(mode="hp", fc=3000)
    print(hp_onepole_coeffs)
    
    noise = np.random.uniform(low=-1, high=1, size=len(SIG))
    bp = Narrow(fs=SR, order=3)
    bp_coeffs = bp.design_filter(mode="bp", fc=5000, bw=10)
    
    tp = TwoZeroTwoPole(fs=SR)
    tp_coeffs = tp.design_filter(mode="notch", fc=2500, bw=5)
    
    print(tp_coeffs)
    
    but1 = Butter(fs=SR, order=6)
    but2 = Butter(fs=SR, order=3)
    but_coeffs = but1.design_filter(mode="lp", fc=1000)
    
    print(but_coeffs)
    
    zav = Zavalishin(fs=SR)
    zav.design_filter(mode="zdf", fc=500)
    
    
    y = np.zeros(len(SIG))
    for i in range(len(SIG)):
        # y[i] = hp_onepole.filt_sample(sample=SIG[i], coeffs=hp_onepole_coeffs)
        # y[i] = comb1.filt_sample(sample=SIG[i])
        # y[i] = bp.filt_sample(sample=noise[i], coeffs=bp_coeffs)
        y[i] = tp.filt_sample(sample=noise[i], coeffs=tp_coeffs)
        z = zav.filt_sample(sample=noise[i])
        y[i] = z[1]
    
    y = tp.filt_frame(frame=SIG, coeffs=tp_coeffs)
    y = bp.filt_frame(frame=SIG, coeffs=bp_coeffs)
    y = lp_onepole.filt_frame(frame=noise, coeffs=lp_onepole_coeffs)
    y = bp.filt_frame(frame=noise, coeffs=bp_coeffs)
    # y = hp_onepole.filt_frame(frame=SIG, coeffs=hp_onepole_coeffs)
    # y = comb1.filt_frame(frame=SIG)
    
    # y = but1.filt_frame(frame=noise, coeffs=but_coeffs)
    # y = but2.filt_frame(frame=y, coeffs=but_coeffs)
    
    sf.write("filt.wav", data=y, samplerate=SR, subtype="PCM_16")
    
    
    # # for sample in x:
    # #     y = bq.bfilt_sample(sample=sample, coeffs=coeffs)
    
    # filtered_frame = bq.filt_frame(frame=SIG, coeffs=coeffs)
    # bq.clear_delayed_samples_cache()
    # sf.write("filtered.wav", data=filtered_frame, samplerate=SR, subtype="PCM_16")
    

# [MAIN PROGRAM]: if the module is being run as the main program, it calls the "main()" function
if __name__ == "__main__":
    main()
