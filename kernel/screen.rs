
/* kernel::screen.rs */
/* A basic screen model & drawing routines */
/*pub mod depth{
    pub type colorDepth = uint;

    pub static black : colorDepth       = 0;
    pub static bw : colorDepth          = 1;
    pub static highcolor : colorDepth   = 16;
    pub static truecolor : colorDepth   = 24;
    pub static rgba : colorDepth        = 32;
}*/

//#[deriving(FromPrimative)]
pub enum ColorDepth{
    NoColor = 0,
    BW = 1,
    HighColor = 16,
    TrueColor = 24,
    RGBA = 32,
}

impl ColorDepth{
    pub fn from_uint(x : uint) -> ColorDepth{
        match x{
            1   => BW,
            16  => HighColor,
            24  => TrueColor,
            32  => RGBA,
            _   => NoColor
        }
    }
}


pub struct Resolution {w : uint, h : uint}

pub mod Resolutions{
    use super::Resolution;
    pub static NO_DISPLAY : Resolution = Resolution{w:0,	h:0};

    pub static QQVGA 	: Resolution = Resolution{w:160,	h:120};
    pub static HQVGA 	: Resolution = Resolution{w:240,	h:160};
    pub static QVGA 	: Resolution = Resolution{w:320,	h:240};
    pub static WQVGA1 	: Resolution = Resolution{w:360,	h:240};
    pub static WQVGA2 	: Resolution = Resolution{w:400,	h:240};
    pub static HVGA 	: Resolution = Resolution{w:480,	h:320};
    pub static VGA 	    : Resolution = Resolution{w:640,	h:480};
    pub static WVGA1 	: Resolution = Resolution{w:720,	h:480};
    pub static WVGA2 	: Resolution = Resolution{w:800,	h:480};
    pub static FWVGA    : Resolution = Resolution{w:854,	h:480};
    pub static SVGA 	: Resolution = Resolution{w:800,	h:600};
    pub static DVGA 	: Resolution = Resolution{w:960,	h:640};
    pub static WSVGA1   : Resolution = Resolution{w:1024,	h:576};
    pub static WSVGA2 	: Resolution = Resolution{w:1024,	h:600};

    pub static XGA 	    : Resolution = Resolution{w:1024,	h:768};
    pub static WXGA1 	: Resolution = Resolution{w:1152,	h:768};
    pub static WXGA2 	: Resolution = Resolution{w:1280,	h:720};
    pub static WXGA3 	: Resolution = Resolution{w:1280,	h:768};
    pub static WXGA4 	: Resolution = Resolution{w:1280,	h:800};
    pub static WXGA5 	: Resolution = Resolution{w:1360,	h:768};
    pub static WXGA6 	: Resolution = Resolution{w:1366,	h:768};
    pub static XGAp 	: Resolution = Resolution{w:1152,	h:864};
    pub static WXGAp 	: Resolution = Resolution{w:1440,	h:900};
    pub static WSXGA 	: Resolution = Resolution{w:1440,	h:960};
    pub static SXGA 	: Resolution = Resolution{w:1280,	h:1024};
    pub static SXGAp 	: Resolution = Resolution{w:1400,	h:1050};
    pub static WSXGAp 	: Resolution = Resolution{w:1680,	h:1050};
    pub static UXGA 	: Resolution = Resolution{w:1600,	h:1200};
    pub static WUXGA 	: Resolution = Resolution{w:1920,	h:1200};

    pub static nHD 	: Resolution = Resolution{w:640,	h:360};
    pub static qHD 	: Resolution = Resolution{w:960,	h:540};
    pub static HD 	: Resolution = Resolution{w:1280,	h:720};
    pub static HDp 	: Resolution = Resolution{w:1600,	h:900};
    pub static FHD 	: Resolution = Resolution{w:1920,	h:1080};
    pub static QHD 	: Resolution = Resolution{w:2560,	h:1440};
    pub static WQXGAp :Resolution = Resolution{w:3200,	h:1800};
    pub static UHD 	: Resolution = Resolution{w:3840,	h:2160};

    pub static Pebble : Resolution = Resolution{w:144,	h:168};
}

pub enum Pixel{
    HighColorPixel(u8, u8, u8),
    TrueColorPixel(u8, u8, u8),
    RGBAPixel(u8,u8,u8,u8)
}

pub trait ScreenCanvas{
    /// Sync with hardware device.
    fn sync(&mut self) -> bool;
    /// Set Resolution. Returns actual Resolution available.
    fn setResolution(&mut self, Resolution) -> Resolution;
    /// Set color depth. Returns actual color depth available.
    fn setColorDepth(&mut self, ColorDepth) -> ColorDepth;

    fn drawPixel(&mut self, &Pixel, &(uint,uint)) -> bool;

    fn flush(&mut self) -> bool;

    /// Check if the device is available
    fn ready(&mut self) -> bool;
}
