.section .text.init
.global _start

_start:
    # 스택 포인터 설정
    la sp, stack_top

    # 다른 하트(코어)는 대기
    csrr a0, mhartid
    bnez a0, spin

    # Rust 메인 함수로 점프
    call rust_main

spin:
    j spin

.section .bss
.align 12
stack_bottom:
    .skip 4096
stack_top: 
