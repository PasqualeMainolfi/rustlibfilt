# import section
import subprocess
import os
import sys

# main scripts
CWD = os.getcwd()
OPS = os.name
_pyft = "pyrustlibfilt/pyft.py" if OPS == "posix" else "pyrustlibfilt\pyft.py"
PYFT_PATH = os.path.join(CWD, _pyft)
IMPORT_STRING = "from rustlibfilt.pyrf import Rustlibfilt\n"

# main function
def main() -> None:
    paths = subprocess.check_output(["python -m site"], shell=True)
    paths = paths.decode().split("\n")
    site_packages_path = [row.strip(", '") for row in paths if row.strip(",").endswith("site-packages'")][0]
    rustlibfilt_lib_path = os.path.join(site_packages_path, "rustlibfilt")
    init_file_path = os.path.join(rustlibfilt_lib_path, "__init__.py")
    
    cmd = "cp" if OPS == "posix" else "copy"
    subprocess.run([cmd, PYFT_PATH, rustlibfilt_lib_path])
    print("[DONE] pyrustlibfilt.pyrf imported")
    
    with open(init_file_path, "r") as file:
        init_file = file.readlines()
    
    with open(init_file_path, "w") as file:
        for line in init_file:
            file.write(line)
        file.write("\n")
        file.write("\n")
        file.write(IMPORT_STRING)
                
    print("[DONE] __init__.py updated\n")
    
# [MAIN PROGRAM]: if the module is being run as the main program, it calls the "main()" function
if __name__ == "__main__":
    main()