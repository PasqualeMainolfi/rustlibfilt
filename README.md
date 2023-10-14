# RUSTLIBFILT  

rustlibfilt è una libreria python scritta in rust per il design e l'implementazione di filtri digitali.  

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

5. import `rustlibfilt` in python and use it

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

**Version 0.1.0**
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

**References**:

- V. Zavalishin, The Art of VA Filter Design, 2018
- Audio EQ Cookbook by Robert Bristow-Johnson, <https://www.musicdsp.org/en/latest/Filters/197-rbj-audio-eq-cookbook.html>  
- <https://www.musicdsp.org/en/latest/Filters/index.html>
- <https://www.dspguide.com/ch19.htm>
- BQD filter design equation, AN2874 Applications note, STMicroelectronics, 2009
