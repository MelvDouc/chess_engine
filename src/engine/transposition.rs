pub(super) enum TpFlag {
    Exact,
    Alpha,
    Beta,
}

pub(super) struct TpEntry {
    score: f64,
    pub(super) depth: usize,
    pub(super) flag: TpFlag,
}

impl TpEntry {
    pub(super) fn create(score: f64, depth: usize, flag: TpFlag) -> Self {
        Self { score, depth, flag }
    }

    pub(super) fn get_score(&self, depth: usize, alpha: f64, beta: f64) -> Option<f64> {
        if self.depth >= depth {
            match self.flag {
                TpFlag::Exact => {
                    return Some(self.score);
                }
                TpFlag::Alpha => {
                    if self.score <= alpha {
                        return Some(alpha);
                    }
                }
                TpFlag::Beta => {
                    if self.score >= beta {
                        return Some(beta);
                    }
                }
            };
        }

        None
    }
}
