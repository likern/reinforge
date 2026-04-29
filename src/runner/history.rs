// #[derive(Debug, Clone)]
// pub struct RewardHistoryElem {
//     id: ActionID,
//     reward: Reward,
// }

// pub struct RunnerHistory {
//     experiments: Experiments,
// }

// impl RunnerHistory {
//     pub fn new(num_exps: usize) -> Self {
//         let hash_map: Experiments = HashMap::with_capacity(num_exps);
//         Self {
//             experiments: hash_map,
//         }
//     }

//     pub fn save(&mut self, experiment_id: usize, experiment: Run) {
//         self.experiments.insert(experiment_id, experiment);
//     }

//     // pub fn dump(&self) -> &Experiments {
//     //     &self.experiments
//     // }
// }
