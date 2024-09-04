use std::{fs::DirEntry, io};



impl crate::config::Config {
    pub fn build(&self) {
        println!("Building...");

        let paths = std::fs::read_dir(&self.src)
            .unwrap()
            .filter_map(|x| x.ok())
            .filter(|x| !self.ignore.contains(&x.path().display().to_string()))
            .collect::<Vec<DirEntry>>();

        // Создать папку build, в её корне создать .obj, туда компилировать асм в объектники, а в корень build уже слинкованный бинарь
        std::fs::create_dir("build");
        std::fs::create_dir("build/.obj");

        // компиляция объектников
        for (i, path) in paths.iter().enumerate() {
            println!("[{}/{}] Name: {}", i+1, paths.len(), path.path().display());
            /*let str_cmd = format!(
                "nasm -f elf64 -o build/.obj/{}.o {}",
                path.as_ref().unwrap().path().file_stem().unwrap().to_str().unwrap(),
                path.as_ref().unwrap().path().display()
            );*/
            //println!("{}", str_cmd);
            
            let flags = self.flags
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<&str>>();

            

            subprocess::Exec::cmd("nasm")
                .args(&flags)
                .arg("-f")
                .arg("elf64")
                .arg("-o")
                .arg(format!("build/.obj/{}.o", path.path().file_stem().unwrap().to_str().unwrap()))
                .arg(path.path())
                .join()
                .unwrap();
        }   

        // Линковка объектников
        println!("Linking...");

        let obj_files = paths.iter()
            .map(|x| format!("build/.obj/{}.o", x.path().file_stem().unwrap().to_str().unwrap()))
            .collect::<Vec<String>>();

        subprocess::Exec::cmd("ld")
            .arg("-o")
            .arg(format!("build/{}", self.name))
            .args(&obj_files)
            .join()
            .unwrap();

        println!("Building successful!");
    }
}