#[cfg(test)]
mod tests {
    use crawler::process;
    use crawler::subcrawlers::{ kontor, general, player };
    // use test::Bencher;
    
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

    // #[bench]
    // fn bench_town(b: &mut Bencher) {
    //     b.iter(|| test_town());
    // }

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
        let date = &general.get_date(None);
        assert!(date == &[11, 05, 1300], "{:?}", date); 
    }

    #[test]
    fn test_player() {
        let process = get_process();
        let player_crawler = player::PlayerCrawler::new(process); 
        let player = player_crawler.get_info(None);
        assert!(player.name.is_some());
        let name = player.name.unwrap();
        assert!(name == "a b", "player: {:?}", name);
        assert!(player.gold == 10000, "gold: {}", player.gold); 
    }

    
    fn get_process() -> process::Process {
        let process = unsafe{process::get_proc_by_name("Patrician3.exe")};
        assert!(process.is_ok());
        process.unwrap()
    }
}
