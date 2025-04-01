# 테스트 방법

## QEMU를 통한 테스트

OS를 QEMU 가상 환경에서 실행하려면 다음 명령어를 사용합니다:

```bash
cargo build
qemu-system-riscv64 -machine virt -bios none -kernel target/riscv64gc-unknown-none-elf/debug/rust-os-1000-lines -nographic
```

## GDB를 통한 디버깅

디버깅을 위해 QEMU와 GDB를 함께 사용하려면:

```bash
# 디버깅 모드로 QEMU 실행
qemu-system-riscv64 -machine virt -bios none -kernel target/riscv64gc-unknown-none-elf/debug/rust-os-1000-lines -nographic -gdb tcp::1234 -S

# 다른 터미널에서 GDB 실행
riscv64-unknown-elf-gdb target/riscv64gc-unknown-none-elf/debug/rust-os-1000-lines -ex "target remote localhost:1234"
```

## 예상 결과

QEMU에서 OS를 성공적으로 실행하면 다음과 같은 출력이 표시됩니다:

```
Rust OS in 1,000 Lines 부팅 중...
OS 초기화 완료!
간단한 셸에 오신 것을 환영합니다!
rust-os> 
```

셸에서 다음 명령어를 테스트할 수 있습니다:
- `help`: 사용 가능한 명령어 목록 표시
- `echo 텍스트`: 입력한 텍스트 출력
- `reboot`: 시스템 재시작 (현재는 메시지만 출력)

## 진행 상태

- [x] 00. Intro: 프로젝트 개요 이해
- [x] 01. Getting Started: 기본 개발 환경 설정
- [x] 02. RISC-V 101: RISC-V 아키텍처 기본 이해
- [x] 03. Overview: 구현할 OS의 전체 아키텍처 파악
- [x] 04. Boot: 부팅 시퀀스 구현
- [x] 05. Hello World!: UART를 통한 출력 구현
- [ ] 06. 메모리 관리: 메모리 할당자 구현
- [ ] 07. 프로세스 관리: 프로세스 및 스케줄링 구현
- [ ] 08. 페이지 테이블: 가상 메모리 구현
- [ ] 09. 파일 시스템: 기본 파일 시스템 구현 
