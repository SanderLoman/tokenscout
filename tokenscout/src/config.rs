use slog::Logger;

pub struct Environment {
    pub analyzer: Analyzer,
    pub database: Database,
    pub ranker: Ranker,
    pub telegram: Telegram,
    pub config: TokenscoutConfig,
    pub logger: Logger,
}
