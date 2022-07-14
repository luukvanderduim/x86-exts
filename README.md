# x86-exts

Finding the extension set used in a binary - fast.

The Stack Overflow question [x86 dissasembler that shows instruction extensions needed](https://stackoverflow.com/questions/59545299/x86-dissasembler-that-shows-instruction-extensions-needed) has a solution that works, but may be slow, depending on your use case.

An answer proposes to use this shell script.

```Bash
#!/bin/bash

REGEX='^([0-9a-f]+)\s+<(.*)>\s+([0-9a-f][0-9a-f]( [0-9a-f][0-9a-f])*)\s+(.*?)$'
EXTS=

while read -r LINE ; do
  if [[ $LINE =~ $REGEX ]] ; then
    ADDR=${BASH_REMATCH[1]}
    LABEL=${BASH_REMATCH[2]}
    HEX=${BASH_REMATCH[3]}
    INSTR=${BASH_REMATCH[5]}
    EXT=$(ZydisInfo -64 $HEX | grep '    ISA-EXT:' | cut -d ' ' -f 6)
    [[ " $EXTS " != *" $EXT "* ]] && EXTS="$EXTS $EXT"
    echo -e "$LABEL\t$EXT\t$INSTR"
  fi
done < <(objdump --disassemble --wide --prefix-addresses --show-raw-insn "$1")

echo "Extensions:$EXTS"
```

This repository solves this problem in two different ways. You may choose either the `brute-force` or the  `self-contained`  approach but the `main`rendition is orders of magnitude faster.

The `brute-force` method is almost a one-on-one translation of the bash script shown above, except that the Rust version does the searching in parallel.

The `main` branch uses the excellent crates [elf-rs](https://github.com/vincenthouyi/elf_rs), to read the elf binary format and lookup  the `.text` section and the [iced-x86](https://github.com/icedland/iced) crate which makes gathering the feature extensions simple.

## Prerequisites

The `main` branch does not require the user to install external programs.

The `brute-force` branch however does require these utilities in path.

`objdump` (as part of GNU Binutils), if you are on some flavour of Linux, this will no doubt be in your distributions  repositories.
You will also need [`Zydis`](https://github.com/zyantific/zydis) on that branch.

## Installation

```Bash
  tinker@cube:~/code$ git clone https://github.com/luukvanderduim/x86-exts.git

    tinker@cube:~/code$ cd x86-exts
    
  tinker@cube:~/code/x86-exts$ cargo install --path=,
```

## Usage

example:

```Bash
tinker@cube:~/$ x86-exts ~/.cargo/bin/rg 
MULTIBYTENOP  SSSE3  CPUID  X64  INTEL186  AVX2  INTEL8086  INTEL286  SSE  INTEL486  PAUSE  SSE2  XSAVE  CET_IBT  INTEL386  CMOV  AVX  
```

## License

*avr-hal* is licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
