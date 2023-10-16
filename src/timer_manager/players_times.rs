

const START_TIME_MINUTES: f64 = 1.;
const START_TIME_SECONDS: f64 = 2.;

#[derive(PartialEq)]
pub enum Player {
    Player1,
    Player2
}

pub struct PlayerTimes<'a> {
    pub name_player: &'a str,
    pub minutes : f64,
    pub seconds : f64
}

impl<'a> PlayerTimes<'a> {
    fn new(name_player: &str) -> PlayerTimes {
        PlayerTimes{
            name_player,
            minutes: START_TIME_MINUTES,
            seconds: START_TIME_SECONDS,
        }
    }
}

pub struct PlayersTimes<'a> {
    pub current_player: Player,
    pub timer_player_1: PlayerTimes<'a>,
    pub timer_player_2: PlayerTimes<'a>
}

impl<'a> PlayersTimes<'a> {
    pub fn new(name_player_1: &'a str, name_player_2: &'a str) -> PlayersTimes<'a> {
        PlayersTimes {
            current_player: Player::Player1,
            timer_player_1: PlayerTimes::new(name_player_1),
            timer_player_2: PlayerTimes::new(name_player_2),
        }
    }

    fn set_name_players(&mut self, name_player_1: &'a str, name_player_2: &'a str) {
        self.timer_player_1.name_player = name_player_1;
        self.timer_player_2.name_player = name_player_2;
    }

    pub fn tick_time(&mut self) -> bool {

        // Current player is player 1
        let mut seconds_current_player = &mut self.timer_player_1.seconds;
        if self.current_player == Player::Player2 {
            seconds_current_player = &mut self.timer_player_2.seconds;
        }

        *seconds_current_player -= 1.;
        if *seconds_current_player < 0. {
            let mut minutes_current_player = &mut self.timer_player_1.minutes;
            if self.current_player == Player::Player2 {
                minutes_current_player = &mut self.timer_player_2.minutes;
            }
            if *minutes_current_player > 0. {
                *minutes_current_player -= 1.;
                *seconds_current_player = 59.;
            } else {
                println!("TIME !!");
                return true;
            }
        }

        println!("Tick, current player time : {}", *seconds_current_player);
        false
    }

    pub fn id_current_player(&self) -> i8 {
        if self.current_player == Player::Player1 { 1 } else { 2 }
    }

    pub fn change_player(&mut self) {
        if self.current_player == Player::Player1 {
            self.current_player = Player::Player2;
        } else {
            self.current_player = Player::Player1;
        }
    }

    pub fn current_player_minutes(&self) -> f64 {
        if self.current_player == Player::Player1 {
            return self.timer_player_1.minutes;
        }
        self.timer_player_2.minutes
    }

    pub fn current_player_seconds(&self) -> f64 {
        if self.current_player == Player::Player1 {
            return self.timer_player_1.seconds;
        }
        self.timer_player_2.seconds
    }
}