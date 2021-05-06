# evm-assembler

An EVM assembly to bytecode converter.

This project is built for fun and my own convenient.

I' ll appreciate it very much if someone tell me some better and easy to use alternatives.

## Usage

you only need `cargo` to build it.

```plain
<binary> <asm-file>
```

or

```plain
cargo r <asm-file>
```

## Example output

```plain
> cargo r shellcode.txt
   Compiling evm-assembler v0.1.0 (D:\evm-assembler)
    Finished dev [unoptimized + debuginfo] target(s) in 1.48s
     Running `target\debug\evm-assembler.exe shellcode.txt`
Assembly Input:

         caller
         push20 0x9B3754c0a0798aDe51e98c7a81aE73aAcf9C2e5F
         xor
         push __get_price__
         jumpi
         push4 0xa6f2ae3a
         push1 0
         mstore
         push1 0
         push1 0
         push1 4
         push1 0x1c
         push1 0
         push20 0xa0379c92AE6533b4C3f82606852E6ACc416DCc3A
         gas
         call
         push1 0
         push1 0
         return
__get_price__:
         jumpdest
         push4 0xe852e741
         push1 0
         mstore
         push1 0x20
         push1 0
         push1 4
         push1 0x1c
         push1 0
         caller
         gas
         call
         push1 0
         mload
         push __sold__
         jumpi
         push1 0x64
         push1 0
         mstore
         push1 0
         return
__sold__:
         jumpdest
         push1 0x63
         push1 0
         mstore
         push1 0x20
         push1 0
         return
    (
        "__get_price__",
    ),
    (
        "__sold__",
        110,
    ),
]

Bytecode Output:

0x33739b3754c0a0798ade51e98c7a81ae73aacf9c2e5f1860485763a6f2ae3a600052600060006004601c600073a0379c92ae6533b4c3f82606852e6acc416dcc3a5af160006000f35b63e852e741600052602060006004601c6000335af1600051606e57606460005260206000f35b606360005260206000f3
```