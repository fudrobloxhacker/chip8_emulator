use chip8_emulator::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    let rom = [0x60, 0x01, 0x70, 0x02, 0xA2, 0x34, 0xD0, 0x15];
    cpu.load_rom(&rom);

    println!("inital state:");
    cpu.print_state();

    for _ in 0..4 {
        let opcode = cpu.fetch();
        println!("exec opcode: 0x{:04X}", opcode);
        cpu.execute(opcode);
        cpu.print_state();
    }
}