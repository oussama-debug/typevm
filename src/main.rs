use typevm::vm::MachineDefinition;

pub fn main() -> Result<(), &'static str> {
    let mut vm = MachineDefinition::new();
    vm.step()
}
