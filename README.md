# RUSTLIBFILT  

A python library, written in Rust, for the design and implentation of digital audio filters  

how to use `rustlibfilt`:

1. install or update rust from <https://forge.rust-lang.org/infra/other-installation-methods.html>  

2. install `maturin` with `cargo`  

    ```sh
    cargo install maturin
    ```

3. create virtual env (follow conda example) and install dependecies  

    ```sh
    conda create -n rpy pyhton=3.11
    conda activate rpy
    ```

4. clone the repo and compile  

    ```sh
    git clone --depth 1 
    ./rustlibfilt
    pip install -r requirements.txt
    make
    ```

5. you can import `rustlibfilt` in python and use it

    ```python
    # import section
    from rustlibfilt import Biquad
    import librosa as lb
    import soundfile as sf

    # main scripts

    PATH = "./audio_files/vox.wav"
    SIG, SR = lb.load(PATH, sr=None)

    # main function
    def main() -> None:
        bq = Biquad(fs=SR)
        coeffs = bq.design_filter(mode="peq", fc=300, q=0.5, dbgain=-12)
        
        # for sample in x:
        #     y = bq.bfilt_sample(sample=sample, coeffs=coeffs)
        
        filtered_frame = bq.filt_frame(frame=SIG, coeffs=coeffs)
        bq.clear_delayed_samples_cache()
        sf.write("filtered.wav", data=filtered_frame, samplerate=SR, subtype="PCM_16")
        

    # [MAIN PROGRAM]: if the module is being run as the main program, it calls the "main()" function
    if __name__ == "__main__":
        main()
    ```  

    Alternatively, you can import `Rustlibfilt` from `rustlibfilt` so that you have auto-completion for functions and their respective documentation.  

    ```python
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

    ```

>Version 0.1.1

1. Alternatively, now you can import `Rustlibfilt` from `rustlibfilt` so that you have auto-completion for functions and their respective documentation.  

    ```python
    from rustlibfilt import Rustlibfilt
    ```  

    read the doc from

    ```python
    print(help(Rustlibfilt))
    ```

2. add new filter types:  
    *Zavalishin*:

    - OnePoleZeroDelay
    - NaiveOnePole
    - TrapIntOnePole
    - StateVariable

    *Butter*:  

    - Lp
    - Hp
    - Bp
    - Notch

3. now you can specify the order of the filter (only for onepole, narrow and butter)

>Version 0.1.0  

Filter types:  
*Biquad*:

- Lp  
- Hp  
- Bp0dB
- Bpsg
- Notch
- Ap
- Peq
- LpShelf
- HpShelf

*Reverb*:  

- CombFIR
- CombIIR
- LowPassFeedbackCombFilterIIR
- Allpass
- LowPassFeedbackAllpassFilterIIR

*One Pole*:

- Lp
- Hp

*DcBlock*:

- DcBlockJulius

*Narrow*:

- Bp
- Notch

*TwoZeroTwoPole*:  

- Bp
- Notch

>References  

- V. Zavalishin, The Art of VA Filter Design, 2018
- Audio EQ Cookbook by Robert Bristow-Johnson, <https://www.musicdsp.org/en/latest/Filters/197-rbj-audio-eq-cookbook.html>  
- <https://www.musicdsp.org/en/latest/Filters/index.html>
- <https://www.dspguide.com/ch19.htm>
- BQD filter design equation, AN2874 Applications note, STMicroelectronics, 2009
- D. Horvath; Z. Cervenanska: J. Kotianova, Digital implementation of Butterworth first-order filter type IIR
