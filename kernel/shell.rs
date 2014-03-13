/* kernel::shell */

use kernel::*;
use kernel::serial::*;
use kernel::screen::*;

// TODO Make the input handlers rely on owned string (~[u8]) rather than raw pointers (?)
//pub type shellInputHandler<'a> = 'a|~[u8], &Shell| -> ();
//pub type shellOutputHandler<'a> = 'a|~[u8], &Shell| -> ();
pub type shellInputHandler<'a> = 'a|char, &Shell| -> ();
pub type shellOutputHandler<'a> = 'a|&str, &Shell| -> ();


// TODO allow lifetime bounds other than static
pub trait Shell{
    fn init(&mut self);
    
    fn attachToSerial(&mut self, &'static Serial) -> bool;
    fn attachToScreen(&mut self, &'static TerminalCanvas) -> bool;

    /// Provide a character of input to theshell (as from a keyboard)
    fn input(&self, char) -> bool;
    /// Provide output from the shell (as a from a program's output)
    fn output(&self, &str) -> bool;
    
    fn addInputHandler(&mut self, shellInputHandler) -> bool;
    fn addOutputHandler(&mut self, shellOutputHandler) -> bool;
}


