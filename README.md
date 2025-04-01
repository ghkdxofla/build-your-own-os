# Rust OS in 1,000 Lines

[Operating System in 1,000 Lines](https://operating-system-in-1000-lines.vercel.app/en/) 웹사이트를 참고하여 Rust로 재구현한 간단한 운영체제입니다.

## 소개

이 프로젝트는 RISC-V 아키텍처를 대상으로 하는 간단한 운영체제를 Rust 언어로 구현합니다. 원래 C로 작성된 가이드를 Rust로 변환한 것입니다.

기본 기능:
- 부팅 및 초기화
- UART 기반 콘솔 I/O
- 메모리 관리
- 프로세스 관리 및 스케줄링
- 간단한 파일 시스템
- 사용자 모드 및 커널 모드 전환
- 시스템 콜 구현

## 시작하기

### 필요 조건

- Rust (nightly 버전)
- RISC-V 툴체인 (예: `riscv64-unknown-elf-gcc`)
- QEMU (RISC-V 아키텍처 지원)

### 설치

```bash
# Rust 설치 (이미 설치되어 있지 않은 경우)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# RISC-V 대상 추가
rustup target add riscv64gc-unknown-none-elf

# 의존성 설치
rustup component add rust-src
```

### 빌드 및 실행

```bash
# 빌드
cargo build --release

# QEMU에서 실행
qemu-system-riscv64 -machine virt -bios none -kernel target/riscv64gc-unknown-none-elf/release/rust-os-1000-lines -nographic
```

## 라이센스

MIT 라이센스에 따라 배포됩니다. 자세한 내용은 [LICENSE](LICENSE) 파일을 참조하세요. 
