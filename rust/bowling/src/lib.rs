#![feature(default_free_fn)]
use std::{default::default, vec};

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameScore {
    OpenFrame(Option<u8>, Option<u8>),
    Strike,
    // Spare,
    TenthFrame(Option<u8>, Option<u8>, Option<u8>),
}

impl FrameScore {
    fn score(&self) -> u16 {
        (match self {
            FrameScore::OpenFrame(Some(s1), Some(s2)) => s1 + s2,
            FrameScore::Strike => 10,
            FrameScore::TenthFrame(Some(s1), Some(s2), option) => match option {
                Some(s3) => s1 + s2 + s3,
                None => s1 + s2,
            },
            _ => 0,
        }) as u16
    }
    fn next_contrib(&self) -> u16 {
        (match self {
            FrameScore::OpenFrame(Some(s1), _) => *s1,
            FrameScore::Strike => 10,
            FrameScore::TenthFrame(Some(s1), _, _) => *s1,
            _ => 0,
        }) as u16
    }
}

#[derive(Debug)]
pub struct BowlingGame {
    frame_count: u8,
    score_history: Vec<FrameScore>,
}

const RUNNING_TENTH: u8 = 9;

impl BowlingGame {
    pub fn new() -> Self {
        default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.frame_count == 10 {
            return Err(Error::GameComplete);
        }
        use FrameScore::*;
        let pins = pins as u8;
        match pins as u8 {
            10 if self.frame_count != RUNNING_TENTH => {
                self.score_history[self.frame_count as usize] = Strike;
                self.frame_count += 1;
                Ok(())
            }
            _ => match &mut self.score_history[self.frame_count as usize] {
                OpenFrame(ref mut s1, ref mut s2) => {
                    if s1.is_none() {
                        *s1 = Some(pins);
                        Ok(())
                    } else if pins + s1.unwrap() <= 10 {
                        *s2 = Some(pins);
                        self.frame_count += 1;
                        Ok(())
                    } else {
                        Err(Error::NotEnoughPinsLeft)
                    }
                }
                TenthFrame(ref mut s1, ref mut s2, ref mut s3) => {
                    if s1.is_none() {
                        *s1 = Some(pins);
                        Ok(())
                    } else if s2.is_none() {
                        if s1.unwrap() == 10 || pins + s1.unwrap() <= 10 {
                            *s2 = Some(pins);
                            if s1.unwrap() != 10 && s2.unwrap() + s1.unwrap() != 10 {
                                self.frame_count += 1
                            }
                            Ok(())
                        } else {
                            *s3 = Some(0);
                            Err(Error::NotEnoughPinsLeft)
                        }
                    } else if s1.unwrap() == 10 {
                        if s2.unwrap() == 10 || s2.unwrap() + pins <= 10 {
                            *s3 = Some(pins);
                            self.frame_count += 1;
                            Ok(())
                        } else {
                            Err(Error::NotEnoughPinsLeft)
                        }
                    } else if s1.unwrap() + s2.unwrap() == 10 {
                        *s3 = Some(pins);
                        self.frame_count += 1;
                        Ok(())
                    } else {
                        Err(Error::GameComplete)
                    }
                }
                _ => {
                    panic!("it hasn't been filled, it cannot be strike");
                }
            },
        }
    }
    pub fn score(&self) -> Option<u16> {
        if self.frame_count == 10 {
            let mut sum = 0;
            for (index, current_frame_score) in self.score_history.iter().enumerate() {
                sum += current_frame_score.score()
                    + match current_frame_score {
                        FrameScore::Strike => match self.score_history.get(index + 1) {
                            Some(&FrameScore::Strike) => {
                                10 + match self.score_history.get(index + 2) {
                                    Some(frame) => frame.next_contrib(),
                                    None => 0,
                                }
                            }
                            Some(&FrameScore::OpenFrame(Some(s1), Some(s2))) => (s1 + s2) as u16,
                            Some(&FrameScore::TenthFrame(Some(s1), Some(s2), _)) => {
                                (s1 + s2) as u16
                            }
                            _ => 0,
                        },
                        FrameScore::OpenFrame(Some(s1), Some(s2)) => {
                            if s1 + s2 == 10 {
                                match self.score_history.get(index + 1) {
                                    Some(frame) => frame.next_contrib(),
                                    None => 0,
                                }
                            } else {
                                0
                            }
                        }
                        _ => 0,
                    }
            }
            Some(sum)
        } else {
            None
        }
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        let mut score_history = vec![FrameScore::OpenFrame(None, None); 9];
        score_history.push(FrameScore::TenthFrame(None, None, None));
        BowlingGame {
            frame_count: 0,
            score_history,
        }
    }
}

/*

example
| 1   | 2      | 3    | 4           | 5          | 6         | 7     | 8        | 9         | 10     |
| --- | ------ | ---- | ----------- | ---------- | --------- | ----- | -------- | --------- | ------ |
| 5 4 | 4 /    | 7 -  | X -         | X -        | X -       | 5 3   | 6 /      | 4 /       | XXX    |
| 9   | 9+10+7 | 26+7 | 33+10+10+10 | 63+10+10+5 | 88+10+5+3 | 106+8 | 114+10+4 | 128+10+10 | 138+30 |
| 9   | 26     | 33   | 63          | 88         | 106       | 114   | 128      | 148       | 178    |
*/
