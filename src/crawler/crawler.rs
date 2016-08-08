use super::process::{ Process };
use super::subcrawlers::{ town, ship, kontor, player, general };
use super::utils::{Output};

pub struct Crawler {
    player: player::PlayerCrawler,
    kontor: kontor::KontorCrawler,
    general: general::GeneralCrawler,
    output: Output
}

impl Crawler {
    pub fn new(process: Process) -> (Crawler, Infos) {
        let mut crawler = Crawler{ player: player::PlayerCrawler::new(process),
                                   kontor: kontor::KontorCrawler::new(process),
                                   general: general::GeneralCrawler::new(process),
                                   output: Output::new() };
        let mut initial_infos = None; 
        while initial_infos == None {
            if let Some(town) =  crawler.general.get_town_name() {
                initial_infos = Some(crawler.get_infos(town, None));
            } 
        }
        (crawler, initial_infos.unwrap())
    }
    
    pub fn crawl(&mut self, old: &Infos) -> Option<Infos> {
        match self.general.get_town_name() {
            Some(town) => self.get_differences(town, old),
            None => {
                self.output.print_if_new("Invalid town".to_string());
                None
            } 
        }
    }

    fn get_differences(&mut self, town_name: String, old: &Infos) -> Option<Infos> {
        let new = self.get_infos(town_name, Some(old));
        let diff = old.diff(&new); 
        if diff.is_empty() {
            Some(diff)
        } else {
            self.output.print_if_new("No changes".to_string());
            None
        }
    }
    
    fn get_infos(&mut self, town_name: String, old: Option<&Infos>) -> Infos {
        let date = self.general.get_date(old); 
        let (town, ship) = self.kontor.get_info(town_name);
        let player = self.player.get_info(old);
        Infos{ date: date, ship: ship, town: town, player: player }
    }
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Infos {
    pub ship: ship::Ship,
    pub town: town::Town,
    pub player: player::Player,
    pub date: [u32; 3]
}
impl Infos {
    pub fn diff(&self, new_infos: &Infos) -> Infos {
        Infos{ date: new_infos.date,
               ship: self.ship.diff(&new_infos.ship),
               town: self.town.diff(&new_infos.town),
               player: self.player.diff(&new_infos.player) }
    }

    pub fn is_empty(&self) -> bool {
        self.ship.is_empty() && self.town.is_empty() && self.player.is_empty()
    }
}
