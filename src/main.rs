use chip8_emulator::cpu::CPU;
use chip8_emulator::display::Display;
use chip8_emulator::keyboard::Keyboard;

fn main() {
    let mut cpu = CPU::new();
    let mut display = Display::new();
    let mut keyboard = Keyboard::new();

    let rom = include_bytes!("../roms/pong.ch8");
    cpu.load_rom(rom);

    println!("intialize state:");
    cpu.print_state();

    for _ in 0..100 {
        let opcode = cpu.fetch();
        println!("exectieng opcode: 0x{:04X}", opcode);
        cpu.execute(opcode, &mut display, &mut keyboard);
        cpu.print_state();
        display.render();
    }
}