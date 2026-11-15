fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        // Эти поля Windows показывает в Диспетчере задач, свойствах файла и списке «Приложения»
        res.set("ProductName", "Aether");
        res.set("FileDescription", "Aether");
        res.set("InternalName", "Aether");
        res.set("OriginalFilename", "aether-browser.exe");
        res.set("CompanyName", "Aether");
        res.set("LegalCopyright", "Aether Browser");
        if let Err(e) = res.compile() {
            eprintln!("cargo:warning=Failed to compile Windows resource: {}", e);
        }
    }
}
