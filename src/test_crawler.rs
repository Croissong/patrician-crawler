#[cfg(test)]
mod tests {
    use super::super::crawler::process::get_proc_by_name;
    use super::super::crawler::addr::get_addresses;
    use super::super::crawler::crawler::Crawler;
    
    #[test]
    fn get_town_ref() {
        let process = get_proc_by_name("Patrician3.exe").unwrap();
        let town_addr = get_addresses(&process).town_name;
        let mut town_name_arr: [u8; 7] = [0u8; 7];
        process.read_memory(&town_addr,
                            &mut town_name_arr as *mut _ as *mut _,
                            7);
        let town_name = &town_name_arr.iter()
            .map(|b| { format!("{}", b.clone() as char) })
            .collect::<String>(); 
        assert!(town_name == &"Luebeck".to_string()); 
    }

    #[test]
    fn test_crawl() {
        let process = get_proc_by_name("Patrician3.exe");
        assert!(process.is_ok());
        let mut crawler = Crawler::new(process.unwrap());
        let infos = crawler.crawl();
        assert!(infos.is_ok());
        assert!(!infos.unwrap().is_empty()); 
    }
        
}
