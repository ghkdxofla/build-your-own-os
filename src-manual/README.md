# 수동 구현 가이드

이 폴더는 여러분이 직접 RISC-V OS를 Rust로 구현하기 위한 폴더입니다.

## 구현해야 할 파일 목록

다음 파일들을 구현해야 합니다:

1. `boot.S` - 부팅 어셈블리 코드
2. `main.rs` - 메인 커널 코드
3. `uart.rs` - UART 통신 모듈
4. `memory.rs` - 메모리 관리 모듈
5. `process.rs` - 프로세스 관리 모듈
6. `fs.rs` - 파일 시스템 모듈

## 참고 자료

각 파일 구현을 위해 [Operating System in 1,000 Lines](https://operating-system-in-1000-lines.vercel.app/en/) 웹사이트를 참고하시면 도움이 됩니다.

## 테스트 방법

구현을 완료한 후, 다음 명령으로 빌드하고 실행할 수 있습니다:

```bash
cargo build --release --features=manual
qemu-system-riscv64 -machine virt -bios none -kernel target/riscv64gc-unknown-none-elf/release/rust-os-1000-lines -nographic
```

테스트는 src와 src-manual 구현이 동일한 기능을 수행하는지 확인하는 데 사용됩니다:

```bash
cargo test --test implementation_comparison
``` 
