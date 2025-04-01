use crate::println;

// 간단한 디스크 I/O 상수
pub const DISK_BASE: usize = 0x1000_1000; // 가상의 디스크 컨트롤러 주소

// 파일 시스템 상수
pub const MAX_FILES: usize = 64;
pub const MAX_FILE_NAME: usize = 28;
pub const BLOCK_SIZE: usize = 512;

// 파일 타입 열거형
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Directory,
}

// inode 구조체 (파일 메타데이터)
#[repr(C)]
pub struct Inode {
    pub file_type: FileType,
    pub size: usize,
    pub direct_blocks: [u32; 12],
    pub indirect_block: u32,
}

impl Inode {
    pub fn new() -> Self {
        Inode {
            file_type: FileType::Regular,
            size: 0,
            direct_blocks: [0; 12],
            indirect_block: 0,
        }
    }
}

// 디렉토리 엔트리 구조체
#[repr(C)]
pub struct DirEntry {
    pub inode_number: u32,
    pub name: [u8; MAX_FILE_NAME],
}

impl DirEntry {
    pub fn new() -> Self {
        DirEntry {
            inode_number: 0,
            name: [0; MAX_FILE_NAME],
        }
    }
    
    pub fn set_name(&mut self, name: &str) {
        let bytes = name.as_bytes();
        let len = core::cmp::min(bytes.len(), MAX_FILE_NAME);
        self.name[..len].copy_from_slice(&bytes[..len]);
    }
    
    pub fn get_name(&self) -> &str {
        let len = self.name.iter().position(|&c| c == 0).unwrap_or(MAX_FILE_NAME);
        core::str::from_utf8(&self.name[..len]).unwrap_or("")
    }
}

// 열린 파일 구조체
pub struct File {
    pub inode_number: u32,
    pub offset: usize,
    pub mode: u32, // 읽기, 쓰기 등
}

impl File {
    pub fn new(inode_number: u32, mode: u32) -> Self {
        File {
            inode_number,
            offset: 0,
            mode,
        }
    }
}

// 간단한 파일 시스템 구현
pub struct FileSystem {
    // 파일 시스템 메타데이터
    initialized: bool,
    inodes: [Option<Inode>; MAX_FILES],
    open_files: [Option<File>; MAX_FILES],
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            initialized: false,
            inodes: [None; MAX_FILES],
            open_files: [None; MAX_FILES],
        }
    }
    
    pub fn init(&mut self) {
        // 파일 시스템 초기화 (나중에 구현)
        self.initialized = true;
        println!("파일 시스템 초기화됨");
    }
    
    // 파일 열기
    pub fn open(&mut self, path: &str, mode: u32) -> Option<usize> {
        // 실제 구현은 나중에
        println!("파일 열기: {}", path);
        None
    }
    
    // 파일 읽기
    pub fn read(&mut self, fd: usize, buf: &mut [u8]) -> Option<usize> {
        // 실제 구현은 나중에
        None
    }
    
    // 파일 쓰기
    pub fn write(&mut self, fd: usize, buf: &[u8]) -> Option<usize> {
        // 실제 구현은 나중에
        None
    }
    
    // 파일 닫기
    pub fn close(&mut self, fd: usize) -> bool {
        // 실제 구현은 나중에
        false
    }
}

// 전역 파일 시스템 인스턴스 (나중에 lazy_static이나 다른 방법으로 개선)
static mut FILESYSTEM: Option<FileSystem> = None;

// 파일 시스템 초기화
pub fn init_filesystem() {
    unsafe {
        FILESYSTEM = Some(FileSystem::new());
        FILESYSTEM.as_mut().unwrap().init();
    }
}

// 파일 시스템 API 함수
pub fn open(path: &str, mode: u32) -> Option<usize> {
    unsafe {
        FILESYSTEM.as_mut()?.open(path, mode)
    }
}

pub fn read(fd: usize, buf: &mut [u8]) -> Option<usize> {
    unsafe {
        FILESYSTEM.as_mut()?.read(fd, buf)
    }
}

pub fn write(fd: usize, buf: &[u8]) -> Option<usize> {
    unsafe {
        FILESYSTEM.as_mut()?.write(fd, buf)
    }
}

pub fn close(fd: usize) -> bool {
    unsafe {
        FILESYSTEM.as_mut().map_or(false, |fs| fs.close(fd))
    }
} 
