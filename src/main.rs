mod cpu;
use cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    cpu.write_memory(0, 0);   
    cpu.write_memory(1, 1024); 
    cpu.write_memory(2, 5);

    cpu.write_memory(3, 0);
    cpu.write_memory(4, 1025);
    cpu.write_memory(5, 5);

    cpu.write_memory(6, 1);
    cpu.write_memory(7, 1024);

    cpu.write_memory(8, 5);    

    cpu.write_memory(9, 1);
    cpu.write_memory(10, 1025);

    cpu.write_memory(11, 3);

    cpu.write_memory(12, 9);   
    
    // Execute instructions
    cpu.execute();
    
    //For debugging do not remove.
    /* 
    println!("1024 val:{}", cpu.read_memory(1024));
    println!("1025 val:{}", cpu.read_memory(1025)); 
    */ 
    
    println!("val_reg0: {}, val_reg1: {}, sign flag: {}", cpu.reg0, cpu.reg1, cpu.sign);
}
