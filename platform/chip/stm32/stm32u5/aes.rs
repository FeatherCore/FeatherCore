//! AES - Advanced Encryption Standard
//! 高级加密标准硬件加速器
//!
//! STM32U5 AES 特性:
//! - 支持 AES-128, AES-192, AES-256 密钥长度
//! - 支持多种加密模式: ECB, CBC, CTR, GCM, CCM
//! - 硬件加密/解密加速
//! - DMA 支持
//! - 支持脱敏加密 (Data Encryption Standard)
//! - 支持 GCM/CCM 认证
//!
//! ## AES 加密模式 / Encryption Modes
//! - **ECB** (Electronic Codebook): 最简单的模式,每个块独立加密
//! - **CBC** (Cipher Block Chaining): 前一个密文块与当前明文块异或后再加密
//! - **CTR** (Counter): 使用计数器生成密钥流
//! - **GCM** (Galois/Counter Mode): 提供加密和认证
//! - **CCM** (Counter with CBC-MAC): 提供加密和认证

/// AES base address / AES 基地址
pub const AES_BASE: usize = 0x420C_0000;

/// AES register offsets / AES 寄存器偏移
pub mod reg {
    /// Control register / 控制寄存器
    pub const CR: usize = 0x00;
    /// Status register / 状态寄存器
    pub const SR: usize = 0x04;
    /// Data input register / 数据输入寄存器
    pub const DINR: usize = 0x08;
    /// Data output register / 数据输出寄存器
    pub const DOUTR: usize = 0x0C;
    /// Key register 0 / 密钥寄存器 0
    pub const KEYR0: usize = 0x10;
    /// Key register 1 / 密钥寄存器 1
    pub const KEYR1: usize = 0x14;
    /// Key register 2 / 密钥寄存器 2
    pub const KEYR2: usize = 0x18;
    /// Key register 3 / 密钥寄存器 3
    pub const KEYR3: usize = 0x1C;
    /// Initialization Vector register 0 / 初始化向量寄存器 0
    pub const IVR0: usize = 0x20;
    /// Initialization Vector register 1 / 初始化向量寄存器 1
    pub const IVR1: usize = 0x24;
    /// Initialization Vector register 2 / 初始化向量寄存器 2
    pub const IVR2: usize = 0x28;
    /// Initialization Vector register 3 / 初始化向量寄存器 3
    pub const IVR3: usize = 0x2C;
    /// Key register 4 (for AES-256) / 密钥寄存器 4 (用于 AES-256)
    pub const KEYR4: usize = 0x30;
    /// Key register 5 (for AES-256) / 密钥寄存器 5 (用于 AES-256)
    pub const KEYR5: usize = 0x34;
    /// Key register 6 (for AES-256) / 密钥寄存器 6 (用于 AES-256)
    pub const KEYR6: usize = 0x38;
    /// Key register 7 (for AES-256) / 密钥寄存器 7 (用于 AES-256)
    pub const KEYR7: usize = 0x3C;
    /// GCM/CCM Initialization Vector register 0 / GCM/CCM 初始化向量寄存器 0
    pub const IVR0_1: usize = 0x40;
    /// GCM/CCM Initialization Vector register 1 / GCM/CCM 初始化向量寄存器 1
    pub const IVR1_1: usize = 0x44;
    /// GCM/CCM Initialization Vector register 2 / GCM/CCM 初始化向量寄存器 2
    pub const IVR2_1: usize = 0x48;
    /// GCM/CCM Initialization Vector register 3 / GCM/CCM 初始化向量寄存器 3
    pub const IVR3_1: usize = 0x4C;
}

/// AES key size / AES 密钥长度
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeySize {
    /// AES-128 (128-bit key) / AES-128 (128位密钥)
    Bits128 = 0b00,
    /// AES-192 (192-bit key) / AES-192 (192位密钥)
    Bits192 = 0b01,
    /// AES-256 (256-bit key) / AES-256 (256位密钥)
    Bits256 = 0b10,
}

/// AES encryption/decryption mode / AES 加密/解密模式
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    /// Electronic Codebook mode / 电子密码本模式
    Ecb = 0b000,
    /// Cipher Block Chaining mode / 密码块链接模式
    Cbc = 0b001,
    /// Counter mode / 计数器模式
    Ctr = 0b010,
    /// Galois/Counter Mode (with authentication) / Galois/计数器模式(带认证)
    Gcm = 0b011,
    /// Counter with CBC-MAC mode (with authentication) / 计数器模式带CBC-MAC(带认证)
    Ccm = 0b100,
}

/// AES operation type / AES 操作类型
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operation {
    /// Encryption / 加密
    Encrypt = 0,
    /// Decryption / 解密
    Decrypt = 1,
}

/// AES data format / AES 数据格式
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataType {
    /// 32-bit data / 32位数据
    Word = 0b00,
    /// 16-bit data (half-word) / 16位数据(半字)
    HalfWord = 0b01,
    /// 8-bit data (byte) / 8位数据(字节)
    Byte = 0b10,
    /// Bit data / 位数据
    Bit = 0b11,
}

/// AES configuration / AES 配置结构体
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Key size / 密钥长度
    pub key_size: KeySize,
    /// Operation mode (encrypt/decrypt) / 操作模式(加密/解密)
    pub operation: Operation,
    /// Cipher mode / 密码模式
    pub mode: Mode,
    /// Data type / 数据类型
    pub data_type: DataType,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            key_size: KeySize::Bits128,
            operation: Operation::Encrypt,
            mode: Mode::Ecb,
            data_type: DataType::Word,
        }
    }
}

/// AES status / AES 状态
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AesStatus {
    /// AES is ready / AES 就绪
    Ready,
    /// AES is busy / AES 忙碌
    Busy,
    /// GCM/CCM authentication phase / GCM/CCM 认证阶段
    AuthPhase,
    /// Error occurred / 发生错误
    Error,
}

/// AES instance / AES 实例
pub struct Aes {
    base: usize,
}

impl Aes {
    /// Create new AES instance / 创建新的 AES 实例
    pub const fn new() -> Self {
        Self { base: AES_BASE }
    }

    /// Enable AES clock in RCC / 在 RCC 中使能 AES 时钟
    fn enable_clock(&self) {
        unsafe {
            let rcc_base = crate::rcc::RCC_BASE as *mut u32;
            let ahb2enr = rcc_base.add(0x4C / 4);
            *ahb2enr |= 1 << 4; // AESEN
        }
    }

    /// Initialize AES with configuration / 使用配置初始化 AES
    pub fn init(&self, config: &Config) {
        self.enable_clock();

        unsafe {
            // Disable AES first / 首先禁用 AES
            let cr = (self.base + reg::CR) as *mut u32;
            core::ptr::write_volatile(cr, 0);

            // Configure AES / 配置 AES
            let mut val = 0u32;
            val |= (config.key_size as u32) << 18;   // KEYSIZE
            val |= (config.mode as u32) << 16;       // MODE
            val |= (config.operation as u32) << 2;   // ALGOMODE
            val |= (config.data_type as u32) << 19;  // DATATYPE
            // Note: EN bit will be set when starting operation
            // 注意: EN 位将在开始操作时设置

            core::ptr::write_volatile(cr, val);
        }
    }

    /// Enable AES / 使能 AES
    pub fn enable(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 0; // EN
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Disable AES / 禁用 AES
    pub fn disable(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val &= !(1 << 0); // EN
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Set encryption key / 设置加密密钥
    /// 
    /// # Arguments
    /// * `key` - Key array (4 words for AES-128, 6 for AES-192, 8 for AES-256)
    ///           密钥数组 (AES-128 用4个32位字, AES-192 用6个, AES-256 用8个)
    pub fn set_key(&self, key: &[u32]) {
        unsafe {
            for (i, &word) in key.iter().enumerate() {
                if i < 8 {
                    let keyr = (self.base + reg::KEYR0 + i * 4) as *mut u32;
                    core::ptr::write_volatile(keyr, word);
                }
            }
        }
    }

    /// Set Initialization Vector (IV) / 设置初始化向量
    /// 
    /// # Arguments
    /// * `iv` - 4 words (128-bit) initialization vector
    ///          4个32位字(128位)初始化向量
    pub fn set_iv(&self, iv: &[u32; 4]) {
        unsafe {
            for (i, &word) in iv.iter().enumerate() {
                let ivr = (self.base + reg::IVR0 + i * 4) as *mut u32;
                core::ptr::write_volatile(ivr, word);
            }
        }
    }

    /// Get AES status / 获取 AES 状态
    pub fn get_status(&self) -> AesStatus {
        unsafe {
            let sr = (self.base + reg::SR) as *mut u32;
            let val = core::ptr::read_volatile(sr);

            if val & (1 << 0) != 0 {
                // BUSY
                if val & (1 << 2) != 0 {
                    AesStatus::AuthPhase
                } else {
                    AesStatus::Busy
                }
            } else if val & 0xE != 0 {
                AesStatus::Error
            } else {
                AesStatus::Ready
            }
        }
    }

    /// Wait for AES operation to complete / 等待 AES 操作完成
    pub fn wait_ready(&self) {
        unsafe {
            let sr = (self.base + reg::SR) as *mut u32;
            while (core::ptr::read_volatile(sr) & (1 << 0)) != 0 {} // Wait for BUSY = 0
        }
    }

    /// Encrypt a single 128-bit block / 加密单个128位块
    /// 
    /// # Arguments
    /// * `input` - 4 words (128-bit) input data / 4个32位字(128位)输入数据
    /// * `output` - 4 words (128-bit) output buffer / 4个32位字(128位)输出缓冲区
    pub fn encrypt_block(&self, input: &[u32; 4], output: &mut [u32; 4]) {
        self.enable();

        unsafe {
            // Write input data / 写入输入数据
            for (i, &word) in input.iter().enumerate() {
                let dinr = (self.base + reg::DINR) as *mut u32;
                core::ptr::write_volatile(dinr, word);
            }

            // Wait for completion / 等待完成
            self.wait_ready();

            // Read output data / 读取输出数据
            for i in 0..4 {
                let doutr = (self.base + reg::DOUTR) as *mut u32;
                output[i] = core::ptr::read_volatile(doutr);
            }
        }
    }

    /// Decrypt a single 128-bit block / 解密单个128位块
    /// 
    /// # Arguments
    /// * `input` - 4 words (128-bit) input data / 4个32位字(128位)输入数据
    /// * `output` - 4 words (128-bit) output buffer / 4个32位字(128位)输出缓冲区
    pub fn decrypt_block(&self, input: &[u32; 4], output: &mut [u32; 4]) {
        self.enable();

        unsafe {
            // Write input data / 写入输入数据
            for (i, &word) in input.iter().enumerate() {
                let dinr = (self.base + reg::DINR) as *mut u32;
                core::ptr::write_volatile(dinr, word);
            }

            // Wait for completion / 等待完成
            self.wait_ready();

            // Read output data / 读取输出数据
            for i in 0..4 {
                let doutr = (self.base + reg::DOUTR) as *mut u32;
                output[i] = core::ptr::read_volatile(doutr);
            }
        }
    }

    /// AES-CBC encryption / AES-CBC 加密
    /// 
    /// # Arguments
    /// * `key` - Encryption key / 加密密钥
    /// * `iv` - Initialization vector (4 words) / 初始化向量(4个字)
    /// * `input` - Input data / 输入数据
    /// * `output` - Output buffer / 输出缓冲区
    pub fn encrypt_cbc(&self, key: &[u32], iv: &[u32; 4], input: &[u8], output: &mut [u8]) {
        self.set_key(key);
        self.set_iv(iv);

        let config = Config {
            key_size: match key.len() {
                4 => KeySize::Bits128,
                6 => KeySize::Bits192,
                _ => KeySize::Bits256,
            },
            operation: Operation::Encrypt,
            mode: Mode::Cbc,
            data_type: DataType::Byte,
        };
        self.init(&config);

        // Process data in 16-byte blocks / 以16字节块处理数据
        let block_size = 16;
        let num_blocks = input.len() / block_size;

        for i in 0..num_blocks {
            let mut block = [0u32; 4];
            for j in 0..4 {
                let offset = i * block_size + j * 4;
                block[j] = u32::from_le_bytes([
                    input[offset],
                    input[offset + 1],
                    input[offset + 2],
                    input[offset + 3],
                ]);
            }

            let mut output_block = [0u32; 4];
            self.encrypt_block(&block, &mut output_block);

            for j in 0..4 {
                let offset = i * block_size + j * 4;
                let bytes = output_block[j].to_le_bytes();
                output[offset] = bytes[0];
                output[offset + 1] = bytes[1];
                output[offset + 2] = bytes[2];
                output[offset + 3] = bytes[3];
            }
        }
    }

    /// AES-CTR encryption / AES-CTR 加密
    /// 
    /// # Arguments
    /// * `key` - Encryption key / 加密密钥
    /// * `nonce` - 4-word nonce/counter / 4个字的随机数/计数器
    /// * `input` - Input data / 输入数据
    /// * `output` - Output buffer / 输出缓冲区
    pub fn encrypt_ctr(&self, key: &[u32], nonce: &[u32; 4], input: &[u8], output: &mut [u8]) {
        self.set_key(key);
        self.set_iv(nonce);

        let config = Config {
            key_size: match key.len() {
                4 => KeySize::Bits128,
                6 => KeySize::Bits192,
                _ => KeySize::Bits256,
            },
            operation: Operation::Encrypt,
            mode: Mode::Ctr,
            data_type: DataType::Byte,
        };
        self.init(&config);

        // Process data in 16-byte blocks / 以16字节块处理数据
        let block_size = 16;
        let num_blocks = (input.len() + block_size - 1) / block_size;

        for i in 0..num_blocks {
            let mut block = [0u32; 4];
            for j in 0..4 {
                block[j] = nonce[j];
            }

            let mut keystream = [0u32; 4];
            self.encrypt_block(&block, &mut keystream);

            for j in 0..4 {
                let block_offset = i * block_size;
                for k in 0..4 {
                    let offset = block_offset + j * 4 + k;
                    if offset < input.len() {
                        output[offset] = input[offset] ^ (keystream[j] >> (k * 8)) as u8;
                    }
                }
            }
        }
    }
}

/// Initialize AES with default configuration / 使用默认配置初始化 AES
pub fn init_aes_default() -> Aes {
    let aes = Aes::new();
    let config = Config::default();
    aes.init(&config);
    aes
}

/// Convenience function for AES-128 ECB encryption / AES-128 ECB 加密的便捷函数
pub fn aes128_ecb_encrypt(key: &[u32; 4], plaintext: &[u8; 16], ciphertext: &mut [u8; 16]) {
    let aes = Aes::new();
    let config = Config {
        key_size: KeySize::Bits128,
        operation: Operation::Encrypt,
        mode: Mode::Ecb,
        data_type: DataType::Byte,
    };
    aes.init(&config);
    aes.set_key(key);

    let mut input = [0u32; 4];
    let mut output = [0u32; 4];
    for i in 0..4 {
        input[i] = u32::from_le_bytes([
            plaintext[i * 4],
            plaintext[i * 4 + 1],
            plaintext[i * 4 + 2],
            plaintext[i * 4 + 3],
        ]);
    }

    aes.encrypt_block(&input, &mut output);

    for i in 0..4 {
        let bytes = output[i].to_le_bytes();
        ciphertext[i * 4] = bytes[0];
        ciphertext[i * 4 + 1] = bytes[1];
        ciphertext[i * 4 + 2] = bytes[2];
        ciphertext[i * 4 + 3] = bytes[3];
    }
}
