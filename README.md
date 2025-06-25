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

## 구현 버전

이 프로젝트는 두 가지 구현 버전을 포함하고 있습니다:

1. **자동 생성 버전(src)**: 
   - AI 도구를 사용하여 자동으로 생성된 버전입니다.
   - `src` 폴더에 구현되어 있습니다.

2. **수동 구현 버전(src-manual)**:
   - 사람이 직접 Rust 코드를 작성해야 하는 버전입니다.
   - `src-manual` 폴더에 템플릿이 준비되어 있으며, 직접 구현해야 합니다.
   - 각 모듈에 대한 템플릿 파일(.template 확장자)을 참고하여 구현하세요.

두 버전 모두 동일한 기능을 구현해야 하며, 테스트를 통해 결과를 비교할 수 있습니다.

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

### src-manual 구현 방법

1. src-manual 폴더의 각 .template 파일을 참고하여 해당 모듈을 구현하세요.
2. 다음 파일들을 작성해야 합니다:
   - `boot.S` - 부팅 코드
   - `main.rs` - 메인 커널 코드
   - `uart.rs` - UART 통신 모듈
   - `memory.rs` - 메모리 관리 모듈
   - `process.rs` - 프로세스 관리 모듈
   - `fs.rs` - 파일 시스템 모듈
3. 구현이 완료되면 아래의 빌드 및 테스트 명령을 실행하세요.

### 빌드 및 실행

#### 자동 생성 버전 (src)

```bash
# 자동 생성 버전 빌드
cargo build --release --features=auto

# QEMU에서 실행
qemu-system-riscv64 -machine virt -bios none -kernel target/riscv64gc-unknown-none-elf/release/rust-os-1000-lines -nographic
```

#### 수동 구현 버전 (src-manual)

```bash
# 수동 구현 버전 빌드
cargo build --release --features=manual

# QEMU에서 실행
qemu-system-riscv64 -machine virt -bios none -kernel target/riscv64gc-unknown-none-elf/release/rust-os-1000-lines -nographic
```

### 테스트 방법

두 구현이 동일한 결과를 내는지 테스트할 수 있습니다:

```bash
# 빌드 도우미 실행
cargo test --test helper_script -- --nocapture

# 구현 비교 테스트 실행 (src 구현이 완료된 후)
cargo test --test implementation_comparison
```

### 버전 비교

두 버전은 각각 다른 프롬프트를 출력합니다:
- 자동 생성 버전: `rust-os>`
- 수동 구현 버전: `rust-os-manual>`

## 라이센스

MIT 라이센스에 따라 배포됩니다. 자세한 내용은 [LICENSE](LICENSE) 파일을 참조하세요. 
