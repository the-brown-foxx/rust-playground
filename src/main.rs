use std::io;

fn main() {
    loop {
        println!("\nEnter library module: ");
        let module = read();

        println!("\nEnter library name: ");
        let name = read();

        println!("\nEnter version name: ");
        let version_name = read();

        print_break();
        println!("libs.versions.toml");

        let module_sections = module.split(":").collect::<Vec<_>>();
        let version = module_sections[2];

        println!("\n[versions]");
        println!("{version_name} = \"{version}\"");

        let module_name = format!("{}:{}", module_sections[0], module_sections[1]);

        println!("\n[libraries]");
        println!("{name} = {{ module = \"{module_name}\", version.ref = \"{version_name}\" }}");

        print_break();
        println!("build.gradle.kts");
        let dependency_name = name.trim().replace("-", ".");
        println!("\nimplementation(libs.{dependency_name})");

        print_break();
    }
}

fn read() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("PANIC!");
    input.trim().to_string()
}

fn print_break() {
    println!("\n{}", "=".repeat(50));
}