//! CORDIC - Coordinate Rotation Digital Computer
//! 坐标旋转数字计算机
//!
//! STM32U5 CORDIC 特性：
//! - 支持多种函数计算：sin, cos, sinh, cosh, atan, atanh, sqrt, ln
//! - 支持 32-bit 和 16-bit 精度模式
//! - 支持 Q31, Q15 数据格式
//! - 支持 DMA 传输
//! - 单次或循环模式

/// CORDIC base address
pub const CORDIC_BASE: usize = 0x4004_0C00;

/// CORDIC register offsets
pub mod reg {
    pub const CSR: usize = 0x00;
    pub const WDATA: usize = 0x04;
    pub const RDATA: usize = 0x08;
}

/// CORDIC function
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Function {
    /// Cosine
    Cosine = 0b00000,
    /// Sine
    Sine = 0b00001,
    /// Phase
    Phase = 0b00010,
    /// Modulus
    Modulus = 0b00011,
    /// Arctangent
    Arctangent = 0b00100,
    /// Hyperbolic cosine
    HyperbolicCosine = 0b10000,
    /// Hyperbolic sine
    HyperbolicSine = 0b10001,
    /// Arctanh
    Arctanh = 0b10100,
    /// Natural logarithm
    NaturalLog = 0b10101,
    /// Square root
    SquareRoot = 0b10110,
}

/// CORDIC precision
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Precision {
    /// 6 iterations (for 16-bit data)
    Iter6 = 0b0001,
    /// 8 iterations
    Iter8 = 0b0010,
    /// 10 iterations
    Iter10 = 0b0011,
    /// 12 iterations
    Iter12 = 0b0100,
    /// 14 iterations
    Iter14 = 0b0101,
    /// 16 iterations
    Iter16 = 0b0110,
    /// 20 iterations
    Iter20 = 0b0111,
    /// 24 iterations (for 32-bit data)
    Iter24 = 0b1000,
    /// 28 iterations
    Iter28 = 0b1001,
    /// 32 iterations
    Iter32 = 0b1010,
    /// 36 iterations
    Iter36 = 0b1011,
    /// 40 iterations
    Iter40 = 0b1100,
    /// 44 iterations
    Iter44 = 0b1101,
    /// 48 iterations
    Iter48 = 0b1110,
    /// 52 iterations
    Iter52 = 0b1111,
}

/// CORDIC scale factor
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scale {
    /// Scale factor 1
    Scale1 = 0b00,
    /// Scale factor 2
    Scale2 = 0b01,
    /// Scale factor 4
    Scale4 = 0b10,
    /// Scale factor 8
    Scale8 = 0b11,
}

/// CORDIC instance
pub struct Cordic;

impl Cordic {
    pub const fn new() -> Self {
        Self
    }

    /// Initialize CORDIC
    pub fn init(&self, func: Function, precision: Precision, scale: Scale) {
        // Enable CORDIC clock
        crate::rcc::enable_ahb1_clock(crate::rcc::ahb1::CORDIC);

        unsafe {
            let csr = (CORDIC_BASE + reg::CSR) as *mut u32;
            let mut val = 0;
            val |= (func as u32) << 0;
            val |= (precision as u32) << 4;
            val |= (scale as u32) << 8;
            val |= 1 << 16; // RRDYIE (result ready interrupt enable)
            core::ptr::write_volatile(csr, val);
        }
    }

    /// Write argument 1
    pub fn write_arg1(&self, value: i32) {
        unsafe {
            let wdata = (CORDIC_BASE + reg::WDATA) as *mut i32;
            core::ptr::write_volatile(wdata, value);
        }
    }

    /// Write argument 2
    pub fn write_arg2(&self, value: i32) {
        unsafe {
            let wdata = (CORDIC_BASE + reg::WDATA) as *mut i32;
            core::ptr::write_volatile(wdata, value);
        }
    }

    /// Read result 1
    pub fn read_res1(&self) -> i32 {
        unsafe {
            let rdata = (CORDIC_BASE + reg::RDATA) as *mut i32;
            core::ptr::read_volatile(rdata)
        }
    }

    /// Read result 2
    pub fn read_res2(&self) -> i32 {
        unsafe {
            let rdata = (CORDIC_BASE + reg::RDATA) as *mut i32;
            core::ptr::read_volatile(rdata)
        }
    }

    /// Check if result ready
    pub fn is_ready(&self) -> bool {
        unsafe {
            let csr = (CORDIC_BASE + reg::CSR) as *mut u32;
            let val = core::ptr::read_volatile(csr);
            (val & (1 << 31)) != 0
        }
    }

    /// Calculate sine
    pub fn sin(&self, angle_q31: i32) -> i32 {
        self.init(Function::Sine, Precision::Iter24, Scale::Scale1);
        self.write_arg1(angle_q31);
        while !self.is_ready() {}
        self.read_res1()
    }

    /// Calculate cosine
    pub fn cos(&self, angle_q31: i32) -> i32 {
        self.init(Function::Cosine, Precision::Iter24, Scale::Scale1);
        self.write_arg1(angle_q31);
        while !self.is_ready() {}
        self.read_res1()
    }

    /// Calculate sine and cosine
    pub fn sin_cos(&self, angle_q31: i32) -> (i32, i32) {
        self.init(Function::Cosine, Precision::Iter24, Scale::Scale1);
        self.write_arg1(angle_q31);
        while !self.is_ready() {}
        (self.read_res2(), self.read_res1()) // (sin, cos)
    }

    /// Calculate arctangent (atan2)
    pub fn atan2(&self, y: i32, x: i32) -> i32 {
        self.init(Function::Arctangent, Precision::Iter24, Scale::Scale1);
        self.write_arg1(x);
        self.write_arg2(y);
        while !self.is_ready() {}
        self.read_res1()
    }

    /// Calculate modulus
    pub fn modulus(&self, x: i32, y: i32) -> i32 {
        self.init(Function::Modulus, Precision::Iter24, Scale::Scale1);
        self.write_arg1(x);
        self.write_arg2(y);
        while !self.is_ready() {}
        self.read_res1()
    }

    /// Calculate square root
    pub fn sqrt(&self, value_q31: i32) -> i32 {
        self.init(Function::SquareRoot, Precision::Iter24, Scale::Scale1);
        self.write_arg1(value_q31);
        while !self.is_ready() {}
        self.read_res1()
    }

    /// Calculate natural logarithm
    pub fn ln(&self, value_q31: i32) -> i32 {
        self.init(Function::NaturalLog, Precision::Iter24, Scale::Scale1);
        self.write_arg1(value_q31);
        while !self.is_ready() {}
        self.read_res1()
    }
}

/// Convert float to Q31 format
pub fn float_to_q31(value: f32) -> i32 {
    (value * (i32::MAX as f32)) as i32
}

/// Convert Q31 to float format
pub fn q31_to_float(value: i32) -> f32 {
    (value as f32) / (i32::MAX as f32)
}

/// Calculate sine in radians
pub fn sin_rad(angle_rad: f32) -> f32 {
    let cordic = Cordic::new();
    let angle_q31 = float_to_q31(angle_rad / core::f32::consts::PI);
    let result_q31 = cordic.sin(angle_q31);
    q31_to_float(result_q31)
}

/// Calculate cosine in radians
pub fn cos_rad(angle_rad: f32) -> f32 {
    let cordic = Cordic::new();
    let angle_q31 = float_to_q31(angle_rad / core::f32::consts::PI);
    let result_q31 = cordic.cos(angle_q31);
    q31_to_float(result_q31)
}

/// Calculate square root
pub fn sqrt_f32(value: f32) -> f32 {
    let cordic = Cordic::new();
    let value_q31 = float_to_q31(value);
    let result_q31 = cordic.sqrt(value_q31);
    q31_to_float(result_q31)
}
