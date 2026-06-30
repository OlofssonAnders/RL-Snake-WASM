use macroquad::prelude::*;
use snake_game::render;
use snake_game::game::{snake_env::SnakeEnv, observation::Observation};
use snake_game::agents::{control::Controller, human_control::HumanController, heuristic_agent::HeuristicAgent, random_agent::RandomAgent, q_learning::QLearningAgent};

struct GameInstance<C> {
    env: SnakeEnv,
    controller: C,
    obs: Observation,
    accumulator: f32,
    step_time: f32,
    wait: i8,
    title: &'static str,
}

impl<C> GameInstance<C>
where
    C: Controller,
{
    fn new(title: &'static str, controller: C) -> Self {
        let env = SnakeEnv::new();
        let obs = env.observe();

        Self {
            env,
            controller,
            obs,
            accumulator: 0.0,
            step_time: 0.1,
            wait: 0,
            title,
        }
    }

    fn update(&mut self, dt: f32) {
        self.accumulator += dt;

        let action = self.controller.choose_action(&self.obs);

        if !self.env.is_done() && self.accumulator >= self.step_time {
            let (new_obs, _reward, _done) = self.env.step(action);
            self.obs = new_obs;
            self.accumulator -= self.step_time;
            self.step_time = self.obs.speed;
        }

        if self.env.is_done() {
            self.wait += 1;
        }
        
        if self.wait > 126 {
            self.obs = self.env.reset();
            self.accumulator = 0.0;
            self.step_time = 0.6;
            self.wait = 0;
        }
    }
}

#[macroquad::main("Snake")]
async fn main() {
    let mut random = GameInstance::new("Random", RandomAgent::new());
    let mut heuristic = GameInstance::new("Heuristic", HeuristicAgent::new());
    let mut qlearning = GameInstance::new("Q-learning", QLearningAgent::new());
    let mut human = GameInstance::new("You", HumanController::new());

    loop {
        let dt = get_frame_time();

        clear_background(BLACK);

        let w = screen_width();
        let h = screen_height();
        let rect = Rect::new(0.,0.,w,h);

        let rects = [
            Rect::new(0.0, 0.0, w / 2.0, h / 2.0),
            Rect::new(w / 2.0, 0.0, w / 2.0, h / 2.0),
            Rect::new(0.0, h / 2.0, w / 2.0, h / 2.0),
            Rect::new(w / 2.0, h / 2.0, w / 2.0, h / 2.0),
        ];

        //heuristic.update(dt);
        //random.update(dt);
        //qlearning.update(dt);
        heuristic.update(dt);

        clear_background(LIGHTGRAY);
        //render::draw_game_in_rect(&heuristic.env.game, rects[0], heuristic.title);
        //render::draw_game_in_rect(&human.env.game, rects[1], human.title);
        //render::draw_game_in_rect(&qlearning.env.game, rects[2], qlearning.title);
        //render::draw_game_in_rect(&random.env.game, rects[3], random.title);
        render::draw_game_in_rect(&heuristic.env.game, rect, heuristic.title);
        next_frame().await;
    }
}