impl crate::config::Config {
    pub fn run(&self) { 
        println!("Running...");
        subprocess::Exec::cmd("build/".to_owned() + &self.name).join().unwrap();
    }
}