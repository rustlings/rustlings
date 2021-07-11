#!/usr/bin/env bash

echo "Let's get you set up with Rustlings!"

echo "Checking requirements..."
if [ -x "$(command -v git)" ]
then
    echo "SUCCESS: Git is installed"
else
    echo "ERROR: Git does not seem to be installed."
    echo "Please download Git using your package manager or over https://git-scm.com/!"
    exit 1
fi

if [ -x "$(command -v cc)" ]
then
    echo "SUCCESS: cc is installed"
else
    echo "ERROR: cc does not seem to be installed."
    echo "Please download (g)cc using your package manager."
    echo "OSX: xcode-select --install"
    echo "Deb: sudo apt install gcc"
    echo "Yum: sudo yum -y install gcc"
    exit 1
fi

if [ -x "$(command -v rustc)" ]
then
    echo "SUCCESS: Rust is installed"
else
    echo "ERROR: Rust does not seem to be installed."
    echo "Please download Rust using https://rustup.rs!"
    exit 1
fi

if [ -x "$(command -v cargo)" ]
then
    echo "SUCCESS: Cargo is installed"
else
    echo "ERROR: Cargo does not seem to be installed."
    echo "Please download Rust and Cargo using https://rustup.rs!"
    exit 1
fi

# Look up python installations, starting with 3 with a fallback of 2
if [ -x "$(command -v python3)" ]
then
    PY="$(command -v python3)"
elif [ -x "$(command -v python)" ]
then
    PY="$(command -v python)"
elif [ -x "$(command -v python2)" ]
then
    PY="$(command -v python2)"
else
    echo "ERROR: No working python installation was found"
    echo "Please install python and add it to the PATH variable"
    exit 1
fi

# Function that compares two versions strings v1 and v2 given in arguments (e.g 1.31 and 1.33.0).
# Returns 1 if v1 > v2, 0 if v1 == v2, 2 if v1 < v2.
function vercomp() {
    if [[ $1 == $2 ]]
    then
        return 0
    fi
    v1=( ${1//./ } )
    v2=( ${2//./ } )
    len1=${#v1[@]}
    len2=${#v2[@]}
    max_len=$len1
    if [[ $max_len -lt $len2 ]]
    then
        max_len=$len2
    fi
    for i in `seq 0 $max_len`
    do
        # Fill empty fields with zeros in v1
        if [ -z "${v1[$i]}" ]
        then
            v1[$i]=0
        fi
        # And in v2
        if [ -z "${v2[$i]}" ]
        then
            v2[$i]=0
        fi
        if [ ${v1[$i]} -gt ${v2[$i]} ]
        then
            return 1
        fi
        if [ ${v1[$i]} -lt ${v2[$i]} ]
        then
            return 2
        fi
    done
    return 0
}

RustVersion=$(rustc --version | cut -d " " -f 2)
MinRustVersion=1.39
vercomp $RustVersion $MinRustVersion
if [ $? -eq 2 ]
then
    echo "ERROR: Rust version is too old: $RustVersion - needs at least $MinRustVersion"
    echo "Please update Rust with 'rustup update'"
    exit 1
else
    echo "SUCCESS: Rust is up to date"
fi

Path=${1:-rustlings/}
echo "Cloning Rustlings at $Path..."
git clone -q https://github.com/rust-lang/rustlings $Path

cd $Path

Version=$(curl -s https://api.github.com/repos/rust-lang/rustlings/releases/latest | ${PY} -c "import json,sys;obj=json.load(sys.stdin);print(obj['tag_name']);")
CargoBin="${CARGO_HOME:-$HOME/.cargo}/bin"

if [[ -z ${Version} ]]
then
    echo "The latest tag version could not be fetched remotely."
    echo "Using the local git repository..."
    Version=$(ls -tr .git/refs/tags/ | tail -1)
    if [[ -z ${Version}  ]]
    then
        echo "No valid tag version found"
        echo "Rustlings will be installed using the main branch"
        Version="main"
    else
        Version="tags/${Version}"
    fi
else
    Version="tags/${Version}"
fi

echo "Checking out version $Version..."
git checkout -q ${Version}

echo "Installing the 'rustlings' executable..."
cargo install --force --path .

if ! [ -x "$(command -v rustlings)" ]
then
    echo "WARNING: Please check that you have '$CargoBin' in your PATH environment variable!"
fi

# Checking whether Clippy is installed.
# Due to a bug in Cargo, this must be done with Rustup: https://github.com/rust-lang/rustup/issues/1514
Clippy=$(rustup component list | grep "clippy" | grep "installed")
if [ -z "$Clippy" ]
then
    echo "Installing the 'cargo-clippy' executable..."
    rustup component add clippy
fi

echo "All done! Run 'rustlings' to get started."
