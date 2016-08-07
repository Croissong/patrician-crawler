#[cfg(test)]
mod tests {
    use crawler::process;
    use crawler::subcrawlers::{ kontor, general }; 
    
    #[test]
    fn test_town() {
        let process = get_process();
        let (town, _) = kontor::KontorCrawler::new(process)
            .get_info("".to_string());
        assert!(&town.total_weight > &0 && &town.total_weight < &10000);
        assert!(town.materials.len() == 20);
        let beer = town.materials.get("beer");
        assert!(beer.is_some());
        assert!(beer.unwrap().sell > 0);
    }

    #[test]
    fn test_ship() {
        let process = get_process();
        let (_, ship) = kontor::KontorCrawler::new(process)
            .get_info("".to_string()); 
        assert!(ship.materials.len() == 20);
        let beer = ship.materials.get("beer");
        assert!(beer.is_some());
        assert!(beer.unwrap().amount == 0);
    }

    #[test]
    fn test_town_name() {
        let process = get_process();
        let general = general::GeneralCrawler::new(process); 
        assert!(&general.get_town_name().expect("") == &"Luebeck".to_string()); 
    }

    #[test]
    fn test_date() {
        let process = get_process();
        let general = general::GeneralCrawler::new(process);
        let date = &general.get_date();
        assert!(date == &[12, 06, 1345], "{:?}", date); 
    }

    
    fn get_process() -> process::Process {
        let process = unsafe{process::get_proc_by_name("Patrician3.exe")};
        assert!(process.is_ok());
        process.unwrap()
    }
}
