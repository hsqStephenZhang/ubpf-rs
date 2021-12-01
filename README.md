# README

## 1. what's ubpf-rs 

Ebpf is a virutal machine runing in kernel space, with bpf type format, which is a risc instructions set. 

This [instruction set](https://github.com/iovisor/bpf-docs/blob/master/eBPF.md) is very simple, so i wonder if i can implement it in userland. I found the [ubpf](https://github.com/iovisor/ubpf) project by bcc. It's straightforward to translate clang into rustlang, so i just work around and have it done.

## 2. how it works

ebpf is basic a interpreter, match all the instructions, handle respectively. And because of the btf(bpf type format) is similar to x86, it's easy to translate btf into x86 instruction set, which is called JIT(just in time compile). By the way, kernel also emit a bunch of native code when bpf's jit is enabled.

## 3. Todos

1. Some less useful instruction is not covered, i am working on it.

2. adjust to api to make it more ergonomic.

3. Bpf verifier, which is a DFS graph search. It's complicated, linux kernel use over 10,000 lines of code to make is a safe sanebox. Currently i donnot have much time to handle it.
