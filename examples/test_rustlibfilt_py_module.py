# import section
from rustlibfilt import Rustlibfilt
import numpy as np
import soundfile as sf



# main scripts
SR = 44100
NOISE = np.random.uniform(-1, 1, SR)
N = len(NOISE)

# main function
def main() -> None:
    bq = Rustlibfilt(fs=SR, family="biquad", mode="lp")
    bq.design_filter(fc=500, q=2)
    print(bq.coeffs)
    
    buffer = int(0.0034 * SR / 2)
    cb = Rustlibfilt(fs=SR, family="harmonic", mode="combi", buffer_lenght=buffer)
    cb.design_filter(t60=.7)
    print(1/0.0034)
    
    
    y = np.zeros(N)
    for i in range(N):
        y[i] = bq.filt_sample(sample=NOISE[i])
    
    bq.clear_delayed_samples_cache()
    y = bq.filt_frame(frame=NOISE)
    
    y = cb.filt_frame(frame=NOISE)

    sf.write("filtered_test.wav", data=y, samplerate=SR, subtype="PCM_16")


# [MAIN PROGRAM]: if the module is being run as the main program, it calls the "main()" function
if __name__ == "__main__":
    main()