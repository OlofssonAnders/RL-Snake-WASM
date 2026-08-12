use macroquad::prelude::*;
use snake_game::agents::q_learning::QLearningSave;
use snake_game::render;
use snake_game::game::{snake_env::SnakeEnv, observation::Observation};
use snake_game::agents::{control::Controller, human_control::HumanController, heuristic_agent::HeuristicAgent, random_agent::RandomAgent, q_learning::QLearningAgent};

const Q_LEARNING_FILE: &'static str = "q_learning";

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn get_requested_agent() -> i32;
}

pub async fn load_q_table(path: &str) -> Result<QLearningAgent, String> {
    let bytes = load_file(path)
        .await
        .map_err(|e| format!("Failed to load {path}: {e}"))?;

    let text = String::from_utf8(bytes)
        .map_err(|e| format!("Invalid UTF-8: {e}"))?;

    let QLearningSave { q, alpha, gamma, epsilon } =
        serde_json::from_str(&text)
            .map_err(|e| format!("Invalid JSON: {e}"))?;

    let agent: QLearningAgent = QLearningAgent::load_save(QLearningSave { q, alpha, gamma, epsilon })
        .map_err(|e| format!("Failed to load Q-learning agent: {e}"))?;

    Ok(agent)
}

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
    let qlearning_agent = load_q_table(Q_LEARNING_FILE).await.expect("Failed to load Q-learning agent");
    let mut qlearning = GameInstance::new("Q-learning", qlearning_agent);
    let mut human = GameInstance::new("You", HumanController::new());

    loop {
        let dt = get_frame_time();

        clear_background(BLACK);

        let w = screen_width();
        let h = screen_height();
        let rect = Rect::new(0.,0.,w,h);

        let requested_agent = 0;

        #[cfg(target_arch = "wasm32")]
        let requested_agent = unsafe {
            get_requested_agent()
        };

        clear_background(LIGHTGRAY);

        match requested_agent {
            0 => {
                random.update(dt);
                render::draw_game_in_rect(&random.env.game, rect, random.title);
                },
            1 => {
                heuristic.update(dt);
                render::draw_game_in_rect(&heuristic.env.game, rect, heuristic.title);
                },
            2 => {
                qlearning.update(dt);
                render::draw_game_in_rect(&qlearning.env.game, rect, qlearning.title);
                },
            3 => {
                human.update(dt);
                render::draw_game_in_rect(&human.env.game, rect, human.title);
                },
            _ => {}
        }
        next_frame().await;
    }
}